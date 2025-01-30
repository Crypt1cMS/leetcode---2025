function isPalindrome(s: string): boolean {
    let result = s.toLowerCase().replace(/[^a-z0-9]/g, "")
    return result === result.split('').reverse().join('')
};