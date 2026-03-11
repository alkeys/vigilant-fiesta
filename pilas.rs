/*
repaso pilas creacion de pila simple
*/

struct Nodo {
    valor: i32,
    siguiente: Option<Box<Nodo>>,
}

struct Pila {
    cima: Option<Box<Nodo>>,
}

// implementacion de metodos para la pila
impl Pila {
    // constructor
    /**
     * Crea una nueva pila
     * @param None
     * @return Pila
     * como funcionar:
     * 1. Inicializa la pila
     * 2. Retorna la pila
     */
    fn nueva() -> Self {
        Pila { cima: None }
    }

    // insertar un elemento
    /**
     * Inserta un elemento al inicio de la pila
     * @param valor
     * @return None
     * como funcionar:
     * 1. Crea un nuevo nodo
     * 2. Inserta el nuevo nodo al inicio de la pila
     */
    fn insertar(&mut self, valor: i32) {
        //crea un nuevo nodo y lo inserta al inicio de la pila
        let nuevo = Box::new(Nodo {
            valor,
            //take() toma el valor de la cima y lo asigna al nuevo nodo
            siguiente: self.cima.take(),
        });

        //asigna el nuevo nodo a la cima
        self.cima = Some(nuevo);
    }

    // eliminar un elemento
    /**
     * Elimina un elemento al inicio de la pila
     * @param None
     * @return Option<i32>
     * como funcionar:
     * 1. Recorre la pila
     * 2. Si encuentra el valor, retorna el nodo
     * 3. Si no encuentra el valor, retorna None
     */
    fn eliminar(&mut self) -> Option<i32> {
        //elimina el primer elemento de la pila
        match self.cima.take() {
            Some(nodo) => {
                //asigna el siguiente nodo a la cima
                self.cima = nodo.siguiente;
                //retorna el valor del nodo eliminado la instruccion Some es para retornar un valor
                Some(nodo.valor)
            }
            None => None,
        }
    }

    // mostrar la pila
    /**
     * Muestra la pila
     * @param None
     * @return None
     * como funcionar:
     * 1. Recorre la pila
     * 2. Imprime el valor de cada nodo
     */
    fn mostrar(&self) {
        let mut actual = &self.cima;

        while let Some(nodo) = actual {
            println!("{}", nodo.valor);
            actual = &nodo.siguiente;
        }
    }
}

fn main() {
    let mut pila = Pila::nueva();
    pila.insertar(1);
    pila.insertar(2);
    pila.insertar(3);
    pila.mostrar();
    pila.eliminar(); // elimina el ultimo elemento
    pila.mostrar();
}
