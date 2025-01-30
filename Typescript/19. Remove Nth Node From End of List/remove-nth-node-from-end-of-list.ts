
class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.next = (next===undefined ? null : next)
    }
}

function removeNthFromEnd(head: ListNode | null, n: number): ListNode | null {

    let length = 0;
    let current = head;

    while (current !== null) {
        length++
        current = current.next
    }
    
    const target = length - n;

    if (target === 0) {
        return head.next
    } 

    current = head;
    
    for (let i = 0; i < target - 1; i++) {
        current = current.next
    }

    if (current && current.next) {
        current.next = current.next.next
    }

    return head
};