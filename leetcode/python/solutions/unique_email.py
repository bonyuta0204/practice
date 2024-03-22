class Solution(object):
    def numUniqueEmails(self, emails):
        """
        :type emails: List[str]
        :rtype: int
        """
        email = set()
        for e in emails:
            email.add(self.stripEmail(e))
        return len(email)

    def stripEmail(self, email):
        local, domain = email.split("@")
        new_local = ""
        for s in local:
            if s == ".":
                continue
            elif s == "+":
                break
            else:
                new_local += s
        return new_local + "@" + domain


if __name__ == "__main__":
    print(Solution().stripEmail("nakamura.yuta+hoge@gmail.com"))
