function code(first, second) {
    return {
        first: `\x1b[${first}m`,
        second: `\x1b[${second}m`
    }
}

function colors(string, code) {
    return `${code.first}${string}${code.second}`
}

colors.bold = function(string) {
    return colors(string, code(1, 22))
}

colors.italic = function(string) {
    return colors(string, code(3, 23))
}

colors.cyan = function(string) {
    return colors(string, code(36, 39));
}