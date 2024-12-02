use std::fs;
use models::{elemento::Categoria, molecula::{self, Ligacao, Molecula, NumeroDeLigacoes, TipoDeLigacao}};
use crate::models::elemento::{Elemento};

mod models;

fn main() {
	let elementos = {
		let filename = "./data/elementos.json"
			.to_string();
		
		let contents = fs::read_to_string(filename)
			.expect("Não foi possível ler o arquivo");

		let dados: Vec<Elemento> = serde_json::
			from_str(&contents)
			.expect("Não foi possível converter os dados");

		dados
	};

	let oxigenio = Elemento::encontrar_por_simbolo(&elementos, "O");
	let hidrogenio = Elemento::encontrar_por_simbolo(&elementos, "H");

	let ametais: Vec<&Elemento> = elementos
		.iter()
		.filter(
			|&el|
				// (el.familia == 16 ||
				// el.get_categoria() == Categoria::Halogenio) &&
				el.simbolo != "O"
				&& el.get_nox() < 0
		)
		.collect();

	ametais
		.iter()
		.for_each(|&ametal| {

			/*
			{	// Hidrácido
				let nome = format!("ácido {}ídrico", ametal.remover_posfix());
				let formula = {
					let valencia = 8 - ametal.camada_de_valencia();
					if valencia <= 1 {
						format!("H{}", ametal.simbolo)
					}	
					else {
						format!("H{}{}", valencia, ametal.simbolo)
					}
				};

				println!("{formula} - {nome}");
			}
			*/

			{	// Oxiácidos
				{	// per{}ico
					let mut molecula = Molecula {atomos: vec![ametal.clone()], ligacoes: vec![]};

					for _ in 0..(8-ametal.camada_de_valencia()) {
						let o = molecula.add_atom(
							oxigenio.clone(), 
							0, 
							TipoDeLigacao::Covalente, 
							NumeroDeLigacoes::Simples
						);
						
						molecula.add_atom(
							hidrogenio.clone(), 
							o,
							TipoDeLigacao::Covalente,
							NumeroDeLigacoes::Simples
						);
					}

					let nox = molecula.get_nox(0, None);
					println!("{nox}");

					for _ in 0..( (8-(if nox<0 { -nox} else {nox}))/2) {
						molecula.add_atom(
							oxigenio.clone(), 
							0,
							TipoDeLigacao::Covalente,
							NumeroDeLigacoes::Dupla
						);
					}

					let mut txt = String::new();
					for a in molecula.atomos {
						txt = format!("{}{}", txt, a.simbolo);
					}

					println!("{txt}");
					
				}
			}
			
		});
}
