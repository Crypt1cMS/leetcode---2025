
function isValid(s: string): boolean {

    let stack: string[] = [];

    for (let eachCharacter of s) {

        if (eachCharacter === "(" || eachCharacter === "{" || eachCharacter === "[") {
            stack.push(eachCharacter)
        }   else if (eachCharacter === ")" && stack.pop() !== "(") {
                return false;
        }   else if (eachCharacter === "}" && stack.pop() !== "{") {
                return false;
        }   else if (eachCharacter === "]" && stack.pop() !== "[") {
                return false;
        }
    } 

    return stack.length === 0

};