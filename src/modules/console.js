let console = new Object()

console.print = function (...print) {
    for (msg of print) {
        Done.core.print(msg)
    }
}

console.println = function (...print) {
    for (msg of print) {
        Done.core.println(msg)
    }
}