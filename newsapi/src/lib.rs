use serde::Deserialize;
use thiserror::Error;
use url::Url;

const BASE_URL: &str = "https://newsapi.org/v2/";


#[derive(Error, Debug)]
pub enum NewsApiError{
    #[error("Download request failed")]
    RequestFailed(#[from] ureq::Error),
    #[error("Conversion of response to string failed")]
    FailedResponseToString(#[from] std::io::Error),
    #[error("Parsing of Article failed")]
    ArticlesParsingFailed(serde_json::Error),
    #[error("Url Parsing failed")]
    UrlParsing(#[from] url::ParseError),
    #[error("Request Failed: {0}")]
    BadRequest(&'static str)
}

#[derive(Deserialize, Debug)]
pub struct NewsAPIResponse{
    status: String,
    pub articles: Vec<Article>,
    code: Option<String>
}
impl NewsAPIResponse {
    pub fn articles(&self) -> &Vec<Article>{
        &self.articles
    }
}

#[derive(Deserialize, Debug)]
pub struct Article{
    title: String,
    url: String,
}
impl Article {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn url(&self) -> &str {
        &self.url
    }
}

// pub fn get_articles(url: &str) -> Result<Articles, NewsApiError>{
//     let response = ureq::get(url).call().map_err(|e| NewsApiError::RequestFailed(e))
//         ?.into_string().map_err(|e| NewsApiError::FailedResponseToString(e))?;

//     let articles: Articles = serde_json::from_str(&response).map_err(|e| NewsApiError::ArticlesParsingFailed(e))?;
//     Ok(articles)
// }

pub enum Endpoint {
    TopHeadlines
}

impl ToString for Endpoint{
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadlines => "top-headlines".to_string()
        }
    }
}

pub enum Country{
    In,
    Us,
}
impl ToString for Country{
    fn to_string(&self) -> String {
        match self {
            Self::In => "in".to_string(),
            Self::Us => "us".to_string(),
        }
    }
}
pub struct NewsAPI{
    api_key: String,
    endpoint: Endpoint,
    country: Country,
}

impl NewsAPI {
    pub fn new(api_key: &str) -> NewsAPI {
        NewsAPI { 
            api_key: api_key.to_string(), 
            endpoint: Endpoint::TopHeadlines, 
            country: Country::In 
        }
    }

    /// Presently only TopHeadlines endpoint is implemented.
    pub fn endpoint(&mut self, endpoint: Endpoint)->&mut NewsAPI{
        self.endpoint = endpoint;
        self
    }

    /// Defines country from where news are to be fetched.
    pub fn country(&mut self, country: Country) -> &mut NewsAPI {
        self.country = country;
        self
    }

    fn prepare_url(&self) -> Result<String, NewsApiError>{
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut().unwrap().push(&self.endpoint.to_string());

        let country = format!("country={}", &self.country.to_string());
        url.set_query(Some(&country));

        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<NewsAPIResponse, NewsApiError>{
        let url = self.prepare_url()?;
        let req = ureq::get(&url).set("Authorization", &self.api_key);
        let response: NewsAPIResponse = req.call()?.into_json()?;

        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_err(response.code))
        }
    }
}

fn map_response_err(code: Option<String>) -> NewsApiError {
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => NewsApiError::BadRequest("Your API Key has been disabled"), 
            _ => NewsApiError::BadRequest("Unknown error")
        }
    } else {
        NewsApiError::BadRequest("Unknown error")
    }
}