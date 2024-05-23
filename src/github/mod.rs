use crate::model::Contribution;
use ::reqwest::blocking::Client;
use anyhow::{bail, Context, Result};
use base64::prelude::*;
use chrono::Datelike;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use std::str::FromStr;

static URL: &str = "https://api.github.com/graphql";

type Date = String;
pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/github_schema.graphql",
    query_path = "src/github/contribution.graphql",
    response_derives = "Debug"
)]
struct ContributionQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/github_schema.graphql",
    query_path = "src/github/id.graphql",
    response_derives = "Debug"
)]
struct IdQuery;

pub struct GithubContributions {
    client: Client,
}

impl GithubContributions {
    pub fn init() -> Self {
        let github_token = std::env::var("GITHUB_API_TOKEN").expect("Missing GITHUB_API_TOKEN");
        let client = Client::builder()
            .user_agent("graphql-rust")
            .default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {github_token}"))
                        .unwrap(),
                ))
                .collect(),
            )
            .build()
            .unwrap();
        GithubContributions { client }
    }

    fn get_user_id(self, username: &str) -> Result<id_query::ResponseData> {
        let variables = id_query::Variables {
            username: username.to_string(),
        };
        let response_body = post_graphql::<IdQuery, _>(&self.client, URL, variables)?;

        response_body.data.context("Failed to fetch data")
    }

    pub fn print_user_id(self, username: &str) {
        let user_id = self.get_user_id(username).unwrap();
        if let Some(user) = user_id.user {
            println!(
                "{:?}",
                String::from_utf8(BASE64_STANDARD.decode(user.id).unwrap())
            );
        }
    }

    pub fn get_contributions(self, user: &str, year: u32) -> Result<Vec<Contribution>> {
        let mut contributions: Vec<Contribution> = vec![];
        let (start, end) = GithubContributions::year_to_git_timestamp(year).unwrap();
        let variables = contribution_query::Variables {
            username: user.to_string(),
            start_date: DateTime::from_str(&start).unwrap(),
            end_date: DateTime::from_str(&end).unwrap(),
        };
        let response_body =
            post_graphql::<ContributionQuery, _>(&self.client, URL, variables).unwrap();

        match response_body.data.unwrap().user {
            Some(user) => {
                let contribution_calendar = user.contributions_collection.contribution_calendar;
                let mut current_week: u32 = 0;
                for week in contribution_calendar.weeks {
                    let mut current_day = 0;
                    for day in week.contribution_days {
                        contributions.push(Contribution {
                            week: current_week,
                            day: current_day,
                            count: day.contribution_count,
                            data: day.color.to_string(),
                        });
                        current_day += 1;
                        if current_day > 7 {
                            bail!("Invalid day {current_day} from date {:?}", day.date);
                        }
                    }
                    current_week += 1;
                    if current_week > 53 {
                        bail!("Invalid week {current_week}");
                    }
                }
            }
            None => {
                bail!("No user found");
            }
        }

        Ok(contributions)
    }

    fn year_to_git_timestamp(year: u32) -> Result<(String, String)> {
        if year < 2008 || year >= chrono::Utc::now().year() as u32 {
            bail!("Invalid Year")
        } else {
            Ok((
                format!("{year}-01-01T00:00:00Z"),
                format!("{year}-12-31T11:59:59Z"),
            ))
        }
    }
}
