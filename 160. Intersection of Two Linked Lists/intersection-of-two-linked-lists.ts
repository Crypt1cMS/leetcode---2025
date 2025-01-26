class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.next = (next===undefined ? null : next)
    }
}

function getIntersectionNode(headA: ListNode | null, headB: ListNode | null): ListNode | null {

    let currentA: ListNode | null = headA
    let currentB: ListNode | null = headB

    while (currentA !== currentB) {
        currentA = currentA ? currentA.next : headB;
        currentB = currentB ? currentB.next : headA
    }

    return currentA

};