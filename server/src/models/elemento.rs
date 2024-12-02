use unidecode::unidecode;
use std::char;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Elemento {
	pub numero_atomico: i32,
	pub nome: String,
	pub simbolo: String,
	categoria: String,
	pub xpos: i32,
	pub ypos: i32,
	pub raio_atomico: Option<f32>,
	pub massa_atomica: f32,
	pub eletronegatividade: Option<f32>,
	pub periodo: i32,
	pub familia: i32,
	fase: char,
	pub configuracao_eletronica: String,
	camadas: String,
	estados_de_oxidacao: String,
	pub carga: Option<i32>
}

pub enum Fase {
	Solido,
	Liquido,
	Gasoso,
	Vazio
}

#[derive(PartialEq, Eq)]
pub enum Categoria {
	Hidrogenio,
	MetalAlcalino,
	MetalAlcalinoTerroso,
	MetalDeTransicao,
	Metaloide,
	OutroMetal,
	Ametal,
	Halogenio,
	GasNobre,
	Lantanideo,
	Actinideo,
	Desconhecido
}

impl Elemento {
	pub fn print(&self) -> String {
		format!("| {:3} | {:02} | {:>16} |",
			self.numero_atomico,
			self.simbolo,
			self.nome
		)
	}

	pub fn get_fase(&self) -> Fase {
		match self.fase {
			'S' => Fase::Solido,
			'L' => Fase::Liquido,
			'G' => Fase::Gasoso,
			_ => Fase::Vazio
		}
	}

	pub fn get_categoria(&self) -> Categoria {
		match self.categoria.as_str() {
			"hidrogênio" => Categoria::Hidrogenio,
			"metal alcalino" => Categoria::MetalAlcalino,
			"metal alcalino terroso" => Categoria::MetalAlcalinoTerroso,
			"metal de transição" => Categoria::MetalDeTransicao,
			"outros metais" => Categoria::OutroMetal,
			"metaloide" => Categoria::Metaloide,
			"ametal" => Categoria::Ametal,
			"halogênio" => Categoria::Halogenio,
			"gás nobre" => Categoria::GasNobre,
			"lantanídeo" => Categoria::Lantanideo,
			"actnídeo" => Categoria::Actinideo,
			"desconhecido" => Categoria::Desconhecido,
			_ => Categoria::Desconhecido
		}
	}

	pub fn remover_posfix(&self) -> String {
		if self.nome == "nitrogênio" {
			return "nitr".to_string();
		}

		if self.nome == "enxofre" {
			return "sulfur".to_string();
		}

		let mut novo = self.nome.clone();
		while let Some(a) = novo.chars().next_back() {
			if "aeiou".contains(a) {
				novo.pop();
			}
			else {
				break;
			}
		}
		unidecode( novo.as_str() ).to_string()
	}

	pub fn get_estados_de_oxidacao(&self) -> Vec<i32> {
		serde_json::
			from_str(&self.estados_de_oxidacao)
			.expect("Não foi possível converter os dados")
	}

	pub fn get_camadas(&self) -> Vec<i32> {
		serde_json::
			from_str(&self.camadas)
			.expect("Não foi possível converter os dados")
	}

	pub fn camada_de_valencia(&self) -> i32 {
		self.get_camadas()[ self.get_camadas().len()-1 ]
	}

	pub fn get_fixed_nox(&self) -> Option<i32> {
		if self.familia == 1 || self.simbolo == "Ag" { return Some(1); }
		if self.familia == 2 || self.simbolo == "Zn" || self.simbolo == "Cd" { return Some(2); }
		if self.get_categoria() == Categoria::Halogenio { return Some(-1); }
		if self.simbolo == "Al" { return Some(3); }
		if self.simbolo == "O" || self.simbolo == "S" { return Some(-2); }
		None
	}

	pub fn get_nox(&self) -> i32 {
		match self.carga {
			Some(e) => e,
			None =>
				match self.get_fixed_nox() {
					Some(d) => d,
					None => self.camada_de_valencia()
				}
		}
	}

	pub fn encontrar_por_simbolo<'a>(lista:&'a Vec<Elemento>, simbolo:&'a str) -> &'a Elemento {
		lista
			.into_iter()
			.find(|&elemento| {
				elemento.simbolo == simbolo
			})
			.expect("Elemento não foi encontrado por símbolo")
	}

	pub fn set_carga(&mut self, nova_carga:i32) {
		self.carga = Some(nova_carga);
	}
}