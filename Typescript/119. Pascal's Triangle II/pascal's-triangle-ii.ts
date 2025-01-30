function getRow(rowIndex: number): number[] {
    
    let result: number[][] = [];

    if (rowIndex === 0) {
        return [1]
    }

    if (rowIndex === 1) {
        return [1,1]
    }

    result = [[1]]

    for (let i = 1; i <= rowIndex; i++) {
        let prevRow = result[i - 1]
        let currentRow: number[] = [1]

        for (let j = 1; j < prevRow.length; j++) {
            currentRow.push(prevRow[j - 1] + prevRow[j])
        }

        currentRow.push(1)
        result.push(currentRow)
    }
    
    return result[rowIndex]
}