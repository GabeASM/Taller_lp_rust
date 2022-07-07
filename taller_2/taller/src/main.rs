
use rand::Rng;

fn main(){
    
    let modelo_1 : Modelo = Modelo::new(generar_respuetas(), 1);
    let modelo_2 : Modelo = Modelo::new(generar_respuetas(), 2);
    let modelo_3 : Modelo = Modelo::new(generar_respuetas() ,3 );
    let modelo_4 : Modelo = Modelo::new(generar_respuetas(),4);

    let modelo_5 : Modelo = Modelo::new(generar_respuetas(), 5);
    let modelo_6 : Modelo = Modelo::new(generar_respuetas(), 6);
    let modelo_7 : Modelo = Modelo::new(generar_respuetas(), 7);
    let modelo_8 : Modelo = Modelo::new(generar_respuetas(),8);
    
    let modelos = vec![modelo_1 , modelo_2  , modelo_3 , modelo_4];
    let respuestas = vec![modelo_5 , modelo_6 , modelo_7 , modelo_8];




    let mut test_1 : Test = Test::new(modelos);
    
    test_1.calcular_modelos();

    println!( "{:?}" , test_1);
    
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
    respuestas : Vec<char>,
    numero_preguntas: usize,
    id :usize
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
    
    fn imprimir(&self){

        println!("modelos {:?}" , self.modelos)

    }
}

impl Modelo{
    fn new(respuestas : Vec<char>, id : usize)->Self{

        let a = respuestas.clone();

        Self{
            respuestas,
            numero_preguntas : a.len(),
            id

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
    let mut rng2 = rand::thread_rng();
    for _i in 0..=rng2.gen_range(0..12){
        let num: usize = rng.gen_range(0..=12);

        respuestas.push(oferta[num])
    }
    respuestas 

}



