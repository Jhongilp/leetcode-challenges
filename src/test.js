/**
 * @param {number[]} candies
 * @param {number} extraCandies
 * @return {boolean[]}
 */


// function getBiggest(arr = []) {
//     let max = 0;
//     arr.forEach(n => {
//         if(n > max) {
//             max = n;
//         }
//     });
//     return max;
// }

 var kidsWithCandies = function(candies, extraCandies) {
    // let biggest = getBiggest(candies);
    let biggest = Math.max(...candies);

    let results = candies.map(c => {
        return c + extraCandies >= biggest
    } )
    return results;
};

// let candies = [4,2,1,1,2];
let candies = [2,3,5,1,3];
let extraCandies = 3;

console.log(kidsWithCandies(candies, extraCandies));