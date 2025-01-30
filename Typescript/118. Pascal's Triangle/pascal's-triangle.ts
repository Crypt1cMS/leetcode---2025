function generate(numRows: number): number[][] {
    
    let result: number[][] = [];

    if (numRows === 1) {
        return [[1]]
    }

    result = [[1]]

    for (let i = 1; i < numRows; i++) {
        let prevRow = result[i - 1]
        let currentRow: number[] = [1]

        for (let j = 1; j < prevRow.length; j++) {
            currentRow.push(prevRow[j - 1] + prevRow[j])
        }

        currentRow.push(1)
        result.push(currentRow)
    }
    
    return result
}