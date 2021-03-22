console.print(require('test@1.1.0')+"\n")
console.println(require('test@1.1.0'))
console.println(colors.cyan('sad'))
//Object
console.println(Object.keys(console))
console.println(Object.keys(colors))
console.println(Object.keys(Done))
console.println(fs.readFileSync('./Cargo.toml'))

let deno = "deno".split('').shift() + "one";
console.println(deno)