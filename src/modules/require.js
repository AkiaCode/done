const require = (moduleString) => {
    const module = moduleString.split("@")

    return "Name: " + module[0] + ", Version: " + module[1]
}