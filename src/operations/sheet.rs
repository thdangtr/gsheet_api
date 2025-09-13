use std::collections::HashMap;

use crate::error::GSheetError;
use crate::models::{
    BatchUpdateValuesResponse, BatchValueRanges, Cell, DateTimeRenderOption, Dimension,
    ValueInputOption, ValueRange, ValueRenderOption,
};
use crate::operations::spreadsheet::SpreadsheetOperations;
use crate::utils::{value_range_to_cells, value_range_to_hash_cell_map};

#[derive(Clone)]
pub struct SheetOperations {
    spreadsheet: SpreadsheetOperations,
    sheet_title: String,
}

impl SheetOperations {
    pub fn new(spreadsheet: SpreadsheetOperations, sheet_title: String) -> Self {
        Self {
            spreadsheet,
            sheet_title,
        }
    }

    pub fn batch_get_value_range(&self) -> BatchGetValueRangeOperations {
        BatchGetValueRangeOperations::new(self)
    }

    pub fn batch_update_value_range(&self) -> BatchUpdateValueRangeOperations {
        BatchUpdateValueRangeOperations::new(self)
    }

    pub fn get_all_value(&self) -> GetAllValueOperations {
        GetAllValueOperations::new(self)
    }

    pub fn get_all_cell(&self) -> GetAllCellOperations {
        GetAllCellOperations::new(self)
    }

    pub fn get_hash_map_cell(&self) -> GetHashMapCellOperations {
        GetHashMapCellOperations::new(self)
    }
}

pub struct BatchGetValueRangeOperations {
    sheet: SheetOperations,
    ranges: Vec<String>,
    major_dimension: Dimension,
    value_render_option: ValueRenderOption,
    date_time_render_option: DateTimeRenderOption,
}

impl BatchGetValueRangeOperations {
    pub fn new(sheet: &SheetOperations) -> Self {
        Self {
            sheet: sheet.clone(),
            ranges: Vec::new(),
            major_dimension: Dimension::default(),
            value_render_option: ValueRenderOption::default(),
            date_time_render_option: DateTimeRenderOption::default(),
        }
    }

    pub fn major_dimension(mut self, dimension: Dimension) -> Self {
        self.major_dimension = dimension;
        self
    }

    pub fn value_render_option(mut self, option: ValueRenderOption) -> Self {
        self.value_render_option = option;
        self
    }

    pub fn date_time_render_option(mut self, option: DateTimeRenderOption) -> Self {
        self.date_time_render_option = option;
        self
    }

    pub fn range(mut self, range: &str) -> Self {
        self.ranges.push(range.to_string());
        self
    }

    pub async fn execute(&self) -> Result<BatchValueRanges, GSheetError> {
        let url = format!(
            "{}/{}/values:batchGet",
            self.sheet.spreadsheet.gsheet_client.base_url, self.sheet.spreadsheet.spreadsheet_id
        );

        let auth_client = self
            .sheet
            .spreadsheet
            .gsheet_client
            .auth_client
            .lock()
            .unwrap();

        let mut request = self
            .sheet
            .spreadsheet
            .gsheet_client
            .client
            .get(&url)
            .bearer_auth(auth_client.get_token())
            .query(&[("majorDimension", self.major_dimension.to_string())])
            .query(&[("valueRenderOption", self.value_render_option.to_string())])
            .query(&[(
                "dateTimeRenderOption",
                self.date_time_render_option.to_string(),
            )]);

        for range in &self.ranges {
            request = request.query(&[("ranges", format!("{}!{}", self.sheet.sheet_title, range))]);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            let value_range: BatchValueRanges = response.json().await?;
            Ok(value_range)
        } else {
            Err(GSheetError::from(response.error_for_status().unwrap_err()))
        }
    }
}

pub struct BatchUpdateValueRangeOperations {
    sheet: SheetOperations,
    value_ranges: Vec<ValueRange>,
    value_input_option: ValueInputOption,
    include_values_in_response: bool,
    response_value_render_option: ValueRenderOption,
    response_date_time_render_option: DateTimeRenderOption,
}

impl BatchUpdateValueRangeOperations {
    pub fn new(sheet: &SheetOperations) -> Self {
        Self {
            sheet: sheet.clone(),
            value_ranges: Vec::new(),
            value_input_option: ValueInputOption::default(),
            include_values_in_response: false,
            response_value_render_option: ValueRenderOption::default(),
            response_date_time_render_option: DateTimeRenderOption::default(),
        }
    }

    pub fn include_values_in_response(mut self, include: bool) -> Self {
        self.include_values_in_response = include;
        self
    }

    pub fn value_input_option(mut self, option: ValueInputOption) -> Self {
        self.value_input_option = option;
        self
    }

    pub fn response_value_render_option(mut self, option: ValueRenderOption) -> Self {
        self.response_value_render_option = option;
        self
    }

    pub fn response_date_time_render_option(mut self, option: DateTimeRenderOption) -> Self {
        self.response_date_time_render_option = option;
        self
    }

    pub fn add_value_range(mut self, range: &str, value: Vec<Vec<String>>) -> Self {
        self.value_ranges.push(ValueRange {
            range: Some(format!("{}!{}", self.sheet.sheet_title, range)),
            values: Some(value),
            major_dimension: Some(Dimension::default()),
        });
        self
    }

    pub async fn execute(&self) -> Result<BatchUpdateValuesResponse, GSheetError> {
        let url = format!(
            "{}/{}/values:batchUpdate",
            self.sheet.spreadsheet.gsheet_client.base_url, self.sheet.spreadsheet.spreadsheet_id
        );

        let auth_client = self
            .sheet
            .spreadsheet
            .gsheet_client
            .auth_client
            .lock()
            .unwrap();

        let body = serde_json::json!({
            "valueInputOption": self.value_input_option,
            "data": self.value_ranges,
            "includeValuesInResponse": self.include_values_in_response,
            "responseValueRenderOption": self.response_value_render_option.to_string(),
            "responseDateTimeRenderOption": self.response_date_time_render_option.to_string(),
        });

        let response = self
            .sheet
            .spreadsheet
            .gsheet_client
            .client
            .post(&url)
            .bearer_auth(auth_client.get_token())
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let result: BatchUpdateValuesResponse = response.json().await?;
            Ok(result)
        } else {
            Err(GSheetError::from(response.error_for_status().unwrap_err()))
        }
    }
}

pub struct GetAllValueOperations {
    sheet: SheetOperations,
    major_dimension: Dimension,
    value_render_option: ValueRenderOption,
    date_time_render_option: DateTimeRenderOption,
}

impl GetAllValueOperations {
    pub fn new(sheet: &SheetOperations) -> Self {
        Self {
            sheet: sheet.clone(),
            major_dimension: Dimension::default(),
            value_render_option: ValueRenderOption::default(),
            date_time_render_option: DateTimeRenderOption::default(),
        }
    }

    pub fn major_dimension(mut self, dimension: Dimension) -> Self {
        self.major_dimension = dimension;
        self
    }

    pub fn value_render_option(mut self, option: ValueRenderOption) -> Self {
        self.value_render_option = option;
        self
    }

    pub fn date_time_render_option(mut self, option: DateTimeRenderOption) -> Self {
        self.date_time_render_option = option;
        self
    }

    pub async fn execute(&self) -> Result<ValueRange, GSheetError> {
        let url = format!(
            "{}/{}/values/{}",
            self.sheet.spreadsheet.gsheet_client.base_url,
            self.sheet.spreadsheet.spreadsheet_id,
            self.sheet.sheet_title
        );

        let auth_client = self
            .sheet
            .spreadsheet
            .gsheet_client
            .auth_client
            .lock()
            .unwrap();

        let request = self
            .sheet
            .spreadsheet
            .gsheet_client
            .client
            .get(&url)
            .bearer_auth(auth_client.get_token())
            .query(&[("majorDimension", self.major_dimension.to_string())])
            .query(&[("valueRenderOption", self.value_render_option.to_string())])
            .query(&[(
                "dateTimeRenderOption",
                self.date_time_render_option.to_string(),
            )]);

        let response = request.send().await?;

        if response.status().is_success() {
            let value_range: ValueRange = response.json().await?;
            Ok(value_range)
        } else {
            Err(GSheetError::from(response.error_for_status().unwrap_err()))
        }
    }
}

pub struct GetAllCellOperations {
    sheet: SheetOperations,
    major_dimension: Dimension,
    value_render_option: ValueRenderOption,
    date_time_render_option: DateTimeRenderOption,
}
impl GetAllCellOperations {
    pub fn new(sheet: &SheetOperations) -> Self {
        Self {
            sheet: sheet.clone(),
            major_dimension: Dimension::default(),
            value_render_option: ValueRenderOption::default(),
            date_time_render_option: DateTimeRenderOption::default(),
        }
    }

    pub fn major_dimension(mut self, dimension: Dimension) -> Self {
        self.major_dimension = dimension;
        self
    }

    pub fn value_render_option(mut self, option: ValueRenderOption) -> Self {
        self.value_render_option = option;
        self
    }

    pub fn date_time_render_option(mut self, option: DateTimeRenderOption) -> Self {
        self.date_time_render_option = option;
        self
    }

    pub async fn execute(&self) -> Result<Vec<Cell>, GSheetError> {
        let value_range = GetAllValueOperations::new(&self.sheet)
            .major_dimension(self.major_dimension.clone())
            .value_render_option(self.value_render_option.clone())
            .date_time_render_option(self.date_time_render_option.clone())
            .execute()
            .await?;

        if value_range.range.is_none() {
            return Err(GSheetError::ResponseParseError("No range found".into()));
        }

        let cells = value_range_to_cells(
            &self.sheet.spreadsheet.spreadsheet_id,
            &self.sheet.sheet_title,
            &value_range,
        )?;
        Ok(cells)
    }
}

pub struct GetHashMapCellOperations {
    sheet: SheetOperations,
    major_dimension: Dimension,
    value_render_option: ValueRenderOption,
    date_time_render_option: DateTimeRenderOption,
}
impl GetHashMapCellOperations {
    pub fn new(sheet: &SheetOperations) -> Self {
        Self {
            sheet: sheet.clone(),
            major_dimension: Dimension::default(),
            value_render_option: ValueRenderOption::default(),
            date_time_render_option: DateTimeRenderOption::default(),
        }
    }

    pub fn major_dimension(mut self, dimension: Dimension) -> Self {
        self.major_dimension = dimension;
        self
    }

    pub fn value_render_option(mut self, option: ValueRenderOption) -> Self {
        self.value_render_option = option;
        self
    }

    pub fn date_time_render_option(mut self, option: DateTimeRenderOption) -> Self {
        self.date_time_render_option = option;
        self
    }

    pub async fn execute(&self) -> Result<HashMap<String, HashMap<usize, Cell>>, GSheetError> {
        let value_range = GetAllValueOperations::new(&self.sheet)
            .major_dimension(self.major_dimension.clone())
            .value_render_option(self.value_render_option.clone())
            .date_time_render_option(self.date_time_render_option.clone())
            .execute()
            .await?;

        if value_range.range.is_none() {
            return Err(GSheetError::ResponseParseError("No range found".into()));
        }

        let hash_map = value_range_to_hash_cell_map(
            &self.sheet.spreadsheet.spreadsheet_id,
            &self.sheet.sheet_title,
            &value_range,
        )?;
        Ok(hash_map)
    }
}
