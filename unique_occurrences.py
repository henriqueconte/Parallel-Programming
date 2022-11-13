# Unique Number of Occurrences (https://leetcode.com/problems/unique-number-of-occurrences/)

# Given an array of integers arr, return true if the number of occurrences of each value in the 
# array is unique, or false otherwise.

class Solution:
    def uniqueOccurrences(self, arr):
        sumDict = {}
        occurencesDict = {}

        # Creates dictionary with each value and their amount of occurrences.
        for element in arr:
            if str(element) in sumDict:
                sumDict[str(element)] += 1
            else:
                sumDict[str(element)] = 1

        # Gets only the values from the dictionary.
        occurrencesCountList = list(sumDict.values())

        # Pushes each amount of occurrences into a new dictionary. If the dictionary already
        # has the key, it stops the execution and returns false, since it has a duplicated occurrence. 
        for element in occurrencesCountList:
            if str(element) in occurencesDict:
                return False
            occurencesDict[str(element)] = 1

        return True

sol = Solution()

print(sol.uniqueOccurrences([-3,0,1,-3,1,1,1,-3,10,0]))