mod theme;
//  export API_KEY=0a48a52136824ffe880210b95879dea0
use std::error::Error;
use newsapi::{NewsAPI, Endpoint, Country, Article};

fn render_articles(articles: &Vec<Article>){
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for a in articles {
        theme.print_text(&format!("`{}`", a.title()));
        theme.print_text(&format!("> *{}*", a.url()));
        theme.print_text("---");
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("API_KEY")?;
    let mut newsapi = NewsAPI::new(&api_key);
    newsapi
        .endpoint(Endpoint::TopHeadlines)
        .country(Country::In);

    // let newsapi_response: NewsAPIResponse = newsapi.fetch()?;
    let newsapi_response = newsapi.fetch_async().await?;

    render_articles(&newsapi_response.articles());

    Ok(())
}