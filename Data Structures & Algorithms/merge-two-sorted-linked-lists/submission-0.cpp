/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

class Solution {
public:
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        ListNode res;
        ListNode* res_cur = &res;

        while (list1 != nullptr && list2 != nullptr) {
            if (list1->val <= list2->val) {
                res_cur->next = list1;
                list1 = list1->next;
            } else {
                res_cur->next = list2;
                list2 = list2->next;
            }

            res_cur = res_cur->next;
        }

        if (list1 == nullptr) {
            res_cur->next = list2;
        } else {
            res_cur->next = list1;
        }

        return res.next;
    }
};
