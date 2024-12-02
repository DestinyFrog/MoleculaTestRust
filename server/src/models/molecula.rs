use serde::{Deserialize, Serialize};
use super::elemento::Elemento;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TipoDeLigacao {
	Covalente,
	CovalenteDativa,
	Ionica,
	Metalica
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum NumeroDeLigacoes {
	Simples = 1,
	Dupla = 2,
	Tripla = 3
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Ligacao {
	tipo: TipoDeLigacao,
	eletrons_compartilhados: NumeroDeLigacoes,
	alfa: usize,
	beta: usize
}

impl Ligacao {
	pub fn new(alfa:usize, beta:usize, tipo:TipoDeLigacao, eletrons_compartilhados:NumeroDeLigacoes) -> Ligacao {
		Ligacao {
			alfa,
			beta,
			eletrons_compartilhados,
			tipo
		}
	}
}

#[derive(Debug)]
pub struct Molecula {
	pub(crate) atomos: Vec<Elemento>,
	pub(crate) ligacoes: Vec<Ligacao>
}

impl Molecula {
	pub fn get_ligacoes(&self, index:usize) -> Vec<&Ligacao> {
		self.ligacoes
			.iter()
			.filter(|&elemento|
				elemento.alfa == index ||
				elemento.beta == index
			)
			.collect()
	}

	pub fn get_nox(&self, index:usize, ignore:Option<&Ligacao>) -> i32 {
		let alvo= &self.atomos[index];
		let mut nox = 0;

		let ligacoes_disponiveis: Vec<&Ligacao> =
		match ignore {
			Some(i) => self
				.get_ligacoes(index)
				.into_iter()
				.filter(|&x| {
					x != i
				})
				.collect(),
			None => self.get_ligacoes(index)
		};

		if ligacoes_disponiveis.len() <= 1 {
			return alvo.get_nox();
		}

		ligacoes_disponiveis
			.iter()
			.for_each(|&ligacao| {
				let my_nox = self.get_nox(
				{
					if &self.atomos[ligacao.alfa] == alvo {
						ligacao.beta
					}
					else {
						ligacao.alfa
					}
				}, Some(ligacao));
				nox += my_nox;
			});
		-nox
	}

	pub fn add_atom(&mut self, elemento:Elemento, from:usize, tipo:TipoDeLigacao, eletrons:NumeroDeLigacoes) -> usize {
		let size = self.atomos.len();
		self.atomos.push(elemento);
		self.ligacoes.push( Ligacao::new(from, size, tipo, eletrons) );
		size
	}
}