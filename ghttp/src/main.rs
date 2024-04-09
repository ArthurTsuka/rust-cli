use clap::{App, Arg};
use reqwest::Client;

#[tokio::main]
async fn main() {
    let matches = App::new("ghttp")
        .version("0.1.0")
        .author("Arthur Tsukamoto")
        .about("Um utilitário similar ao curl em Rust")
        .arg(
            Arg::with_name("URL")
                .help("URL para fazer a solicitação HTTP")
                .required(true)
                .index(1),
        )
        .get_matches();

    let url = matches.value_of("URL").unwrap();
    
    let client = Client::new();
	match client.get(url).send().await {
		Ok(response) => {
			println!("Status: {}", response.status());
			println!("Headers:\n{:#?}", response.headers());
			match response.text().await {
				Ok(body) => println!("Body:\n{}", body),
				Err(e) => eprintln!("Erro ao obter o corpo da resposta: {:?}", e),
			}
		}
		Err(e) => eprintln!("Erro ao fazer a solicitação: {:?}", e),
	}
}

