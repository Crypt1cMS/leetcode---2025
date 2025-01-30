function finalPositionOfSnake(n: number, commands: string[]): number {
    let row = 0;
    let col = 0;

    for (let command of commands) {
        switch (command) {
            case "UP":
                row--;
                break;
            case "DOWN":
                row++;
                break;
            case "LEFT":
                col--;
                break;
            case "RIGHT":
                col++;
                break;
        }
    }

    return (row * n) + col
};