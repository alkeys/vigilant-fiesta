# 🎓 Guía para Estudiantes: Estructuras de Datos en Rust

---

## 🧠 1. Conceptos Previos: La Memoria en Rust

Antes de ver cómo se conectan los nodos, necesitamos entender las herramientas que Rust nos da para manejar la memoria de forma segura. A diferencia de C o C++, en Rust **no existen los punteros nulos (`null`)** ni la liberación manual de memoria (`free`).

### 📦 `Box<T>` (Memoria Dinámica)

Las estructuras de datos como las listas pueden crecer o encogerse. Su tamaño no se conoce al compilar el programa.
`Box` le dice a Rust: _"Guarda este dato en el Heap (memoria dinámica) y dame un puntero seguro hacia él"_.
En nuestras estructuras, un `Nodo` contiene a otro `Nodo`. Si no usáramos `Box`, Rust intentaría calcular un tamaño infinito de memoria.

### 🤔 `Option<T>` (Puede existir o no)

Como no hay `null`, ¿cómo decimos que una lista está vacía o que llegamos al último elemento? ¡Con `Option`!

- **`Some(valor)`**: Significa que **sí** hay un dato.
- **`None`**: Significa que **no** hay dato (equivalente seguro a `null`).

Nuestros nodos siempre apuntan al `siguiente` usando `Option<Box<Nodo>>`. Si es `None`, es el final del camino.

### 🪄 El método mágico: `.take()`

Imagina que tienes una caja con una manzana (`Some(manzana)`) y quieres sacar la manzana pero dejar la caja intacta y vacía (`None`) para que el programa no falle. Eso hace `.take()`. Es vital al momento de insertar o eliminar nodos sin causar errores de memoria.

---

## 📚 2. Las Estructuras de Datos

### A. La Pila (Stack) - `pilas.rs`

**Concepto:** Último en entrar, primero en salir (LIFO - Last In, First Out).
**Ejemplo de la vida real:** El botón de "Atrás" en tu navegador web. Pilas y deshaces las últimas páginas visitadas.

🛑 **Lo que debes observar en el código:**
Presta atención a cómo la función `insertar` crea un nuevo nodo y lo coloca exactamente en la **`cima`**, empujando al que antes era la cima hacia abajo usando `siguiente: self.cima.take()`.

---

### B. Lista Simplemente Enlazada - `ListaSimple.rs`

**Concepto:** Una cadena de elementos. Para llegar al tercer elemento, debes pasar obligatoriamente por el primero y el segundo.
**Ejemplo de la vida real:** Una búsqueda del tesoro donde cada pista te da la dirección de la siguiente.

🛑 **Lo que debes observar en el código:**
A diferencia de la pila, mira la función `buscar`. Fíjate cómo usa `&self` (una referencia). Esto significa que la función solo "pide prestada" la lista para mirarla, pero no tiene permiso para modificarla ni eliminarla. Esto se llama **Borrowing** en Rust.

---

### C. Lista Circular Simple - `ListaCircularSimple.rs`

**Concepto:** Igual que la lista simple, pero el último elemento apunta de regreso al primero, formando un ciclo.
**Ejemplo de la vida real:** El turno en un juego de mesa de 4 jugadores. Después del jugador 4, le toca de nuevo al jugador 1.

🛑 **Lo que debes observar en el código:**
En el código de repaso, la estructura base es lineal. ¡Ese es el truco de Rust!
Debido a la regla de **Ownership** (Propiedad), un `Box` solo puede tener **un dueño**. El primer nodo no puede ser "dueño" de la lista y al mismo tiempo ser propiedad del puntero "siguiente" del último nodo.
Para hacer un ciclo real en Rust, tendríamos que usar herramientas avanzadas de múltiples dueños compartidos y mutabilidad interior, como `Rc<RefCell<Nodo>>`.

---

## 💻 3. Retos Prácticos para ti

¡La mejor forma de aprender es rompiendo y mejorando código! Intenta resolver estos retos modificando los archivos `.rs`:

1. **Reto en `pilas.rs`:** ¿Puedes crear una función `fn tamaño(&self) -> usize` que cuente y te devuelva cuántos elementos hay en la pila sin eliminarlos? _(Pista: fíjate cómo lo hace el método mostrar)._
2. **Reto en `ListaSimple.rs`:** Modifica el método `insertar` para que, en lugar de agregar al inicio (como pila), recorra toda la lista y agregue el nuevo elemento **al final** (como cola).
3. **Reto en `ListaSimple.rs`:** Crea un método `eliminar_valor(&mut self, valor: i32)` que busque un número específico en la lista y elimine ese nodo en particular reconectando la cadena.

---
