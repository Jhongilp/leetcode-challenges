/**
 * @param {number} n
 * @return {number}
 */
var arrangeCoins = function(n) {
    let coinsLeft = n;
    let r = 0;
    for(let i = 0; i <= coinsLeft; i++) {
       coinsLeft -= i;
       r = i;
       console.log(coinsLeft);
    }
    
    console.log("steps: ", r);
    return r;
};

arrangeCoins(5);

// 0 - 5
// 1 - 4
// 2 - 2
// 3 - 2