// importa Rc (Reference Counted)
// permite que varios punteros sean dueños del mismo dato
use std::rc::Rc;

// importa RefCell
// permite modificar datos incluso cuando hay referencias compartidas
use std::cell::RefCell;


// definición de la estructura Nodo
struct Nodo {

    // valor que guarda el nodo
    valor: i32,

    // puntero al siguiente nodo
    // Option -> puede ser Some(nodo) o None
    // Rc -> permite que varios nodos apunten al mismo nodo
    // RefCell -> permite modificar el nodo en tiempo de ejecución
    siguiente: Option<Rc<RefCell<Nodo>>>,
}


// estructura de la lista circular
struct ListaCircular {

    // puntero al primer nodo de la lista
    cabeza: Option<Rc<RefCell<Nodo>>>,

    // puntero al último nodo
    cola: Option<Rc<RefCell<Nodo>>>,

    // cantidad de elementos de la lista
    size: usize,
}


// implementación de métodos para ListaCircular
impl ListaCircular {

    // función que crea una nueva lista vacía
    fn nueva() -> Self {

        // retorna una nueva instancia de ListaCircular
        ListaCircular {

            // la lista inicia sin nodos
            cabeza: None,

            // tampoco hay último nodo
            cola: None,

            // tamaño inicial es 0
            size: 0,
        }
    }


    // función para insertar un nodo al final
    fn insertar(&mut self, valor: i32) {

        // crea un nuevo nodo en memoria compartida
        let nuevo = Rc::new(RefCell::new(Nodo {

            // guarda el valor recibido
            valor,

            // inicialmente no apunta a nadie
            siguiente: None,
        }));


        // toma la cola actual y la deja en None temporalmente
        match self.cola.take() {

            // caso donde la lista ya tiene nodos
            Some(vieja_cola) => {

                // el nodo que era el último ahora apunta al nuevo nodo
                vieja_cola.borrow_mut().siguiente = Some(nuevo.clone());

                // el nuevo nodo ahora se convierte en la cola
                self.cola = Some(nuevo.clone());

                // se conecta la cola con la cabeza para mantener la circularidad
                self.cola
                    .as_ref()          // obtiene referencia a la cola
                    .unwrap()          // saca el Option (sabemos que existe)
                    .borrow_mut()      // permite modificar el nodo
                    .siguiente = self.cabeza.clone(); // apunta a la cabeza
            }


            // caso donde la lista está vacía
            None => {

                // el nuevo nodo se convierte en la cabeza
                self.cabeza = Some(nuevo.clone());

                // también es la cola
                self.cola = Some(nuevo.clone());

                // el nodo apunta a sí mismo
                // esto crea el ciclo en la lista circular
                nuevo.borrow_mut().siguiente = Some(nuevo.clone());
            }
        }

        // aumenta el tamaño de la lista
        self.size += 1;
    }


    // función para mostrar los elementos de la lista
    fn mostrar(&self) {

        // si la lista está vacía
        if self.cabeza.is_none() {

            // termina la función
            return;
        }

        // empieza el recorrido desde la cabeza
        let mut actual = self.cabeza.clone();


        // recorre exactamente la cantidad de nodos
        // esto evita un bucle infinito
        for _ in 0..self.size {

            // si existe un nodo
            if let Some(nodo) = actual {

                // imprime el valor del nodo
                print!("{} -> ", nodo.borrow().valor);

                // se mueve al siguiente nodo
                actual = nodo.borrow().siguiente.clone();
            }
        }

        // mensaje final indicando que la lista vuelve al inicio
        println!("(vuelve al inicio)");
    }
}


// función principal del programa
fn main() {

    // crea una nueva lista circular vacía
    let mut lista = ListaCircular::nueva();

    // inserta el valor 1
    lista.insertar(1);

    // inserta el valor 2
    lista.insertar(2);

    // inserta el valor 3
    lista.insertar(3);

    // muestra la lista
    lista.mostrar();
}