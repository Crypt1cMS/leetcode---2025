function convertDateToBinary(date: string): string {
    
    let dateSplit = date.split("-")
    let arr = []

    for (let binary of dateSplit) {
        let num = Number(binary)
        arr.push(num.toString(2))
    }

    return arr.join("-")

};