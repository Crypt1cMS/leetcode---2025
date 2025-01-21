function maximumWealth(accounts: number[][]): number {
    
    let result = [];

    for (let i = 0; i < accounts.length; i++) {

        let num = 0

        for (let account of accounts[i]) {
            num += account;
        }

        result.push(num)
    }

    return Math.max(...result)

};