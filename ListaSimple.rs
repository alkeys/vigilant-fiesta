/*
lista simple creacion de lista simple
*/

// Definición de un nodo de la lista
struct Nodo {
    valor: i32, // valor que guarda el nodo (un número entero)

    // puntero al siguiente nodo
    // Option significa que puede haber un nodo (Some) o no haber (None)
    // Box guarda el nodo en el heap (memoria dinámica)
    //la diferencia entre box y Rc<RefCell<Nodo>> es que box es un puntero de propiedad
    //y Rc<RefCell<Nodo>> es un puntero de referencia
    siguiente: Option<Box<Nodo>>,
}

// estructura principal de la lista
pub struct Lista {
    // primer nodo de la lista (inicio o cabeza)
    // puede ser Some(nodo) o None si la lista está vacía
    cabezera: Option<Box<Nodo>>,

    // tamaño de la lista (cantidad de nodos)
    size: usize,
}

// implementación de los métodos de la lista
impl Lista {
    // constructor de la lista
    /**
     * Crea una nueva lista
     */
    fn nueva() -> Self {
        // retorna una lista vacía
        Lista {
            // no hay nodos todavía
            cabezera: None,

            // tamaño inicial 0
            size: 0,
        }
    }

    // insertar un elemento al inicio de la lista
    fn insertar(&mut self, valor: i32) {
        // crea un nuevo nodo en memoria dinámica
        let nuevo = Box::new(Nodo {
            // guarda el valor recibido
            valor,

            // el siguiente nodo será la antigua cabeza
            // take() toma el valor y deja None en la cabeza temporalmente
            siguiente: self.cabezera.take(),
        });

        // ahora el nuevo nodo se convierte en la cabeza
        self.cabezera = Some(nuevo);

        // aumenta el tamaño de la lista en 1
        self.size += 1;
    }

    // eliminar un elemento al inicio de la lista
    fn eliminar(&mut self) -> Option<i32> {
        // match revisa si hay nodo en la cabeza
        match self.cabezera.take() {
            // si existe un nodo
            Some(nodo) => {
                // la cabeza ahora será el siguiente nodo
                self.cabezera = nodo.siguiente;

                // reduce el tamaño de la lista
                self.size -= 1;

                // retorna el valor del nodo eliminado
                Some(nodo.valor)
            }

            // si la lista está vacía
            None => None,
        }
    }

    // mostrar todos los elementos de la lista
    fn mostrar(&self) {
        // referencia al primer nodo
        let mut actual = &self.cabezera;

        // mientras exista nodo
        while let Some(nodo) = actual {
            // imprime el valor del nodo
            println!("{}", nodo.valor);

            // pasa al siguiente nodo
            actual = &nodo.siguiente;
        }
    }

    // buscar un valor dentro de la lista
    fn buscar(&self, valor: i32) -> Option<&Nodo> {
        // empieza desde la cabeza
        let mut actual = &self.cabezera;

        // recorre todos los nodos
        while let Some(nodo) = actual {
            // si el valor coincide
            if nodo.valor == valor {
                // retorna referencia al nodo encontrado
                return Some(nodo);
            }

            // pasa al siguiente nodo
            actual = &nodo.siguiente;
        }

        // si no encontró el valor retorna None
        None
    }
}

// función principal del programa
fn main() {
    // crea una nueva lista vacía
    let mut lista = Lista::nueva();

    // inserta el valor 1 en la lista
    lista.insertar(1);

    // inserta el valor 2
    lista.insertar(2);

    // inserta el valor 3
    lista.insertar(3);

    // muestra la lista completa
    lista.mostrar();

    // elimina el primer nodo (la cabeza)
    lista.eliminar();

    // muestra la lista nuevamente después de eliminar
    lista.mostrar();
}
