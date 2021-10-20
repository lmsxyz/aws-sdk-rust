// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Provides information about a forecast. Returned as part of the <a>QueryForecast</a> response.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Forecast {
    /// <p>The forecast.</p>
    /// <p>The <i>string</i> of the string-to-array map is one of the following
    /// values:</p>
    /// <ul>
    /// <li>
    /// <p>p10</p>
    /// </li>
    /// <li>
    /// <p>p50</p>
    /// </li>
    /// <li>
    /// <p>p90</p>
    /// </li>
    /// </ul>
    pub predictions: std::option::Option<
        std::collections::HashMap<std::string::String, std::vec::Vec<crate::model::DataPoint>>,
    >,
}
impl std::fmt::Debug for Forecast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Forecast");
        formatter.field("predictions", &self.predictions);
        formatter.finish()
    }
}
/// See [`Forecast`](crate::model::Forecast)
pub mod forecast {
    /// A builder for [`Forecast`](crate::model::Forecast)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) predictions: std::option::Option<
            std::collections::HashMap<std::string::String, std::vec::Vec<crate::model::DataPoint>>,
        >,
    }
    impl Builder {
        /// Adds a key-value pair to `predictions`.
        ///
        /// To override the contents of this collection use [`set_predictions`](Self::set_predictions).
        ///
        /// <p>The forecast.</p>
        /// <p>The <i>string</i> of the string-to-array map is one of the following
        /// values:</p>
        /// <ul>
        /// <li>
        /// <p>p10</p>
        /// </li>
        /// <li>
        /// <p>p50</p>
        /// </li>
        /// <li>
        /// <p>p90</p>
        /// </li>
        /// </ul>
        pub fn predictions(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::vec::Vec<crate::model::DataPoint>>,
        ) -> Self {
            let mut hash_map = self.predictions.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.predictions = Some(hash_map);
            self
        }
        /// <p>The forecast.</p>
        /// <p>The <i>string</i> of the string-to-array map is one of the following
        /// values:</p>
        /// <ul>
        /// <li>
        /// <p>p10</p>
        /// </li>
        /// <li>
        /// <p>p50</p>
        /// </li>
        /// <li>
        /// <p>p90</p>
        /// </li>
        /// </ul>
        pub fn set_predictions(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<
                    std::string::String,
                    std::vec::Vec<crate::model::DataPoint>,
                >,
            >,
        ) -> Self {
            self.predictions = input;
            self
        }
        /// Consumes the builder and constructs a [`Forecast`](crate::model::Forecast)
        pub fn build(self) -> crate::model::Forecast {
            crate::model::Forecast {
                predictions: self.predictions,
            }
        }
    }
}
impl Forecast {
    /// Creates a new builder-style object to manufacture [`Forecast`](crate::model::Forecast)
    pub fn builder() -> crate::model::forecast::Builder {
        crate::model::forecast::Builder::default()
    }
}

/// <p>The forecast value for a specific date. Part of the <a>Forecast</a>
/// object.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DataPoint {
    /// <p>The timestamp of the specific forecast.</p>
    pub timestamp: std::option::Option<std::string::String>,
    /// <p>The forecast value.</p>
    pub value: std::option::Option<f64>,
}
impl std::fmt::Debug for DataPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DataPoint");
        formatter.field("timestamp", &self.timestamp);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`DataPoint`](crate::model::DataPoint)
pub mod data_point {
    /// A builder for [`DataPoint`](crate::model::DataPoint)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) timestamp: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<f64>,
    }
    impl Builder {
        /// <p>The timestamp of the specific forecast.</p>
        pub fn timestamp(mut self, input: impl Into<std::string::String>) -> Self {
            self.timestamp = Some(input.into());
            self
        }
        /// <p>The timestamp of the specific forecast.</p>
        pub fn set_timestamp(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.timestamp = input;
            self
        }
        /// <p>The forecast value.</p>
        pub fn value(mut self, input: f64) -> Self {
            self.value = Some(input);
            self
        }
        /// <p>The forecast value.</p>
        pub fn set_value(mut self, input: std::option::Option<f64>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`DataPoint`](crate::model::DataPoint)
        pub fn build(self) -> crate::model::DataPoint {
            crate::model::DataPoint {
                timestamp: self.timestamp,
                value: self.value,
            }
        }
    }
}
impl DataPoint {
    /// Creates a new builder-style object to manufacture [`DataPoint`](crate::model::DataPoint)
    pub fn builder() -> crate::model::data_point::Builder {
        crate::model::data_point::Builder::default()
    }
}
