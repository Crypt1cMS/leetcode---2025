function countPoints(rings: string): number {
    
    let myMap = new Map();
    let count = 0;

    for (let i = 0; i < rings.length; i+= 2) {
        let color = rings[i]
        let rod = parseInt(rings[i + 1])

        if (!myMap.has(rod)) {
            myMap.set(rod, new Set())
        }
        
        myMap.get(rod)!.add(color)
    };

    for (let value of myMap.values()) {
        if (value.size === 3) {
            count++
        }
    }

    return count

};