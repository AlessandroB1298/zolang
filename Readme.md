# ZoLang


## Overview
This project is for a new language based off of typescript & C++ syntax, compiled by rust, and then converted to a WASM lib.


## Getting Started

Using the exmaple.zo, we can compile using the following command:
`cargo run ./example.zo`


```
const func shuffleCard::vector<Number> = (cards :: vector<Number>) => {
	// randomizes cards

};

const func dealCards::vector<<Number>> = (cards: vector<Number>) => {
	//deals 3 cards

};

const func playGame::void = (cards::vector<<Number>>) =>{
	//game loop

};

const func root:Main = () => {

	//func-chain
	const cards::vector<Number> = {1,2,3,4,5,6,7,8,9};


	shuffleCards(cards).dealCards().playGame();
}

```

## Func-Chain?
The idea is very simple,  lets say we have a function that returns a vector of Numbers, and we know we would like to sequentially execute some functions afterwords, with func chaining, async/await is implied implicitly, meaning when we pass cards to the shuffleCards function, it will then take the expected output of shuffle cards and pass it down to the other functions, so for example


## Initial Notes

### Syntax:
```
 - White space does not matter, similar to c++ or java.

 - To declare a data type use the ::
	const cards::vector<<Number>> = {};

-We can also use the single colon ':' to for special types, such as a Main function or our starting point.

- For types we can do something interesting for example, The Number Type:

	Number : {
		int : {}
		float : {}
	}

	We can do something like:
		const x::Number:int = 1;
		const y::Number:float = 1.2;


```
