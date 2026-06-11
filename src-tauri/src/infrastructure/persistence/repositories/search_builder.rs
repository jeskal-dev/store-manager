use serde_json::Value;
use sqlx::{QueryBuilder, Sqlite};

use crate::shared::criteria::{Operator, Sort, SortOrder};

pub fn push_filter_condition(builder: &mut QueryBuilder<Sqlite>, field: &str, operator: &Operator) {
    builder.push(field).push(" ");
    match operator {
        Operator::Eq => { builder.push("= "); }
        Operator::Ne => { builder.push("!= "); }
        Operator::Gt => { builder.push("> "); }
        Operator::Gte => { builder.push(">= "); }
        Operator::Lt => { builder.push("< "); }
        Operator::Lte => { builder.push("<= "); }
        Operator::Like | Operator::Ilike => { builder.push("LIKE "); }
        Operator::In => { builder.push("IN "); }
    }
}

pub fn push_filter_value(builder: &mut QueryBuilder<Sqlite>, value: &Value, operator: &Operator) {
    match (operator, value) {
        (Operator::In, Value::Array(arr)) => {
            builder.push("(");
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    builder.push(", ");
                }
                bind_value(builder, item);
            }
            builder.push(")");
        }
        _ => bind_value(builder, value),
    }
}

pub fn push_sort(builder: &mut QueryBuilder<Sqlite>, sort: &[Sort]) {
    if sort.is_empty() {
        return;
    }
    builder.push(" ORDER BY ");
    for (i, s) in sort.iter().enumerate() {
        if i > 0 {
            builder.push(", ");
        }
        builder.push(&s.field).push(" ");
        match s.order {
            SortOrder::Asc => { builder.push("ASC"); }
            SortOrder::Desc => { builder.push("DESC"); }
        }
    }
}

fn bind_value(builder: &mut QueryBuilder<Sqlite>, value: &Value) {
    match value {
        Value::String(s) => {
            builder.push_bind(s.clone());
        }
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                builder.push_bind(i);
            } else if let Some(f) = n.as_f64() {
                builder.push_bind(f);
            } else {
                builder.push_bind(n.to_string());
            }
        }
        Value::Bool(b) => {
            builder.push_bind(*b);
        }
        Value::Null => {
            builder.push_bind(None::<String>);
        }
        _ => {
            builder.push_bind(value.to_string());
        }
    }
}
