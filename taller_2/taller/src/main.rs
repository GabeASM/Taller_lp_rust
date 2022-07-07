
use rand::Rng;

fn main(){
    let modelo_1 : Modelo = Modelo::new(generar_respuetas());
    let modelo_2 : Modelo = Modelo::new(generar_respuetas());
    let modelo_3 : Modelo = Modelo::new(generar_respuetas());
    let modelo_4 : Modelo = Modelo::new(generar_respuetas());

    let modelo_5 : Modelo = Modelo::new(generar_respuetas());
    let modelo_6 : Modelo = Modelo::new(generar_respuetas());
    let modelo_7 : Modelo = Modelo::new(generar_respuetas());
    let modelo_8 : Modelo = Modelo::new(generar_respuetas());
    
    let modelos = vec![modelo_1 , modelo_2  , modelo_3 , modelo_4];
    let respuestas = vec![modelo_5 , modelo_6 , modelo_7 , modelo_8];



    let mut test_1 : Test = Test::new(modelos);
    
    test_1.calcular_modelos();
    
    println!("{:?}" , test_1.calcular_correctas(&respuestas));

    
  
}

#[derive(Debug)]
struct Test{
    
    modelos : Vec<Modelo>, 
    cantidad : usize,
    respuestas_correctas : u16
}
#[derive(Debug)]
struct Modelo{
    respuestas : Vec<char>
}

impl Test{
    fn new(modelos: Vec<Modelo>)->Self{
        Self{
            modelos,
            cantidad : 0,
            respuestas_correctas : 0
        }
    }
    fn calcular_correctas(&mut self , modelos_respuestas : &Vec<Modelo>) -> usize{

       for i in 0..self.cantidad{
          
       }
        let correctas = self.modelos.iter()
        .zip(&*modelos_respuestas)
        .filter(|&(modelos, modelos_respuestas)| modelos.retornar_respuestas() == modelos_respuestas.retornar_respuestas() )
        .count();
        
        correctas

    }
    fn calcular_modelos(&mut self){
        self.cantidad = self.modelos.len();
    }    
}

impl Modelo{
    fn new(respuestas : Vec<char>)->Self{
        Self{
            respuestas
        }
    }

    fn retornar_respuestas(&self)->Vec<char>{    
        let a = self.respuestas.clone();
        a
    }
}

fn generar_respuetas()->Vec<char>{

    let oferta = vec!['a', 'b', 'c' ,'d','a', 'b', 'c' ,'d','a', 'b', 'c' ,'d','a', 'b', 'c' ,'d'];
    let mut respuestas : Vec<char> = vec![];
    let mut rng = rand::thread_rng();
    for _i in 0..=12{
        let num: usize = rng.gen_range(0..=12);

        respuestas.push(oferta[num])
    }
    respuestas

}



