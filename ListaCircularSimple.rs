/*
lista circular simple creacion de lista circular simple
*/

// estructura que representa un nodo de la lista
struct Nodo {
    // valor almacenado en el nodo (entero)
    valor: i32,

    // puntero al siguiente nodo
    // Option significa que puede existir nodo o no
    // Some(nodo) -> hay nodo
    // None -> no hay nodo

    // Box guarda el nodo en memoria dinámica (heap)

    // en una lista circular real el último nodo apuntaría al primero
    siguiente: Option<Box<Nodo>>,
}

// estructura de la lista circular
struct ListaCircular {
    // puntero al primer nodo de la lista
    // si la lista está vacía será None
    cabeza: Option<Box<Nodo>>,

    // cantidad de nodos en la lista
    size: usize,
}

// implementación de los métodos de la lista circular
impl ListaCircular {
    /**
     * Crea una nueva lista circular
     */

    // constructor de la lista
    fn nueva() -> Self {
        // se crea una lista vacía
        ListaCircular {
            // no hay nodos todavía
            cabeza: None,

            // tamaño inicial 0
            size: 0,
        }
    }

    // insertar un nodo al inicio de la lista
    fn insertar(&mut self, valor: i32) {
        // se crea un nuevo nodo en el heap
        let nuevo = Box::new(Nodo {
            // se guarda el valor recibido
            valor,

            // el siguiente nodo será la antigua cabeza
            // take() toma el valor y deja None temporalmente
            siguiente: self.cabeza.take(),
        });

        // el nuevo nodo pasa a ser la cabeza
        self.cabeza = Some(nuevo);

        // aumenta el tamaño de la lista
        self.size += 1;
    }

    /**
     * Elimina un elemento al inicio de la lista
     */

    fn eliminar(&mut self) -> Option<i32> {
        // match revisa si existe un nodo en la cabeza
        match self.cabeza.take() {
            // si hay nodo
            Some(nodo) => {
                // la cabeza ahora será el siguiente nodo
                self.cabeza = nodo.siguiente;

                // disminuye el tamaño de la lista
                self.size -= 1;

                // retorna el valor del nodo eliminado
                Some(nodo.valor)
            }

            // si la lista está vacía
            None => None,
        }
    }

    /**
     * Muestra todos los nodos de la lista
     */

    fn mostrar(&self) {
        // variable temporal para recorrer la lista
        // empieza en la cabeza
        let mut actual = &self.cabeza;

        // mientras exista nodo
        while let Some(nodo) = actual {
            // imprime el valor del nodo
            println!("{}", nodo.valor);

            // pasa al siguiente nodo
            actual = &nodo.siguiente;
        }
    }
}

// función principal del programa
fn main() {
    // crea una lista circular vacía
    let mut lista = ListaCircular::nueva();

    // inserta el valor 1
    lista.insertar(1);

    // inserta el valor 2
    lista.insertar(2);

    // inserta el valor 3
    lista.insertar(3);

    // muestra los elementos de la lista
    lista.mostrar();

    // elimina el primer nodo
    lista.eliminar();

    // muestra la lista después de eliminar
    lista.mostrar();
}
