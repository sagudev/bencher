use std::str::FromStr;

use bencher_json::alert::{JsonAlert, JsonSide};
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, SqliteConnection};
use dropshot::HttpError;
use uuid::Uuid;

use super::{statistic::QueryStatistic, QueryThreshold};
use crate::{model::perf::QueryPerf, schema, schema::alert as alert_table, util::map_http_error};

#[derive(Queryable)]
pub struct QueryAlert {
    pub id: i32,
    pub uuid: String,
    pub perf_id: i32,
    pub threshold_id: i32,
    pub statistic_id: i32,
    pub side: bool,
    pub boundary: f32,
    pub outlier: f32,
}

impl QueryAlert {
    pub fn get_id(conn: &mut SqliteConnection, uuid: impl ToString) -> Result<i32, HttpError> {
        schema::alert::table
            .filter(schema::alert::uuid.eq(uuid.to_string()))
            .select(schema::alert::id)
            .first(conn)
            .map_err(map_http_error!("Failed to get alert."))
    }

    pub fn get_uuid(conn: &mut SqliteConnection, id: i32) -> Result<Uuid, HttpError> {
        let uuid: String = schema::alert::table
            .filter(schema::alert::id.eq(id))
            .select(schema::alert::uuid)
            .first(conn)
            .map_err(map_http_error!("Failed to get alert."))?;
        Uuid::from_str(&uuid).map_err(map_http_error!("Failed to get alert."))
    }

    pub fn into_json(self, conn: &mut SqliteConnection) -> Result<JsonAlert, HttpError> {
        let Self {
            id: _,
            uuid,
            perf_id,
            threshold_id,
            statistic_id,
            side,
            boundary,
            outlier,
        } = self;
        Ok(JsonAlert {
            uuid: Uuid::from_str(&uuid).map_err(map_http_error!("Failed to get alert."))?,
            perf: QueryPerf::get_uuid(conn, perf_id)?,
            threshold: QueryThreshold::get_uuid(conn, threshold_id)?,
            statistic: QueryStatistic::get_uuid(conn, statistic_id)?,
            side: Side::from(side).into(),
            boundary: boundary.into(),
            outlier: outlier.into(),
        })
    }
}

pub enum Side {
    Left = 0,
    Right = 1,
}

impl From<bool> for Side {
    fn from(side: bool) -> Self {
        match side {
            false => Self::Left,
            true => Self::Right,
        }
    }
}

impl Into<bool> for Side {
    fn into(self) -> bool {
        match self {
            Self::Left => false,
            Self::Right => true,
        }
    }
}

impl Into<JsonSide> for Side {
    fn into(self) -> JsonSide {
        match self {
            Self::Left => JsonSide::Left,
            Self::Right => JsonSide::Right,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = alert_table)]
pub struct InsertAlert {
    pub uuid: String,
    pub perf_id: i32,
    pub threshold_id: i32,
    pub statistic_id: i32,
    pub side: bool,
    pub boundary: f32,
    pub outlier: f32,
}
