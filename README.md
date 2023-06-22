# Problem Manager (stores solutions, with categories for fast searching)

### Description 

One of my main hobbies is competitive programming, I have solved at hundreds if not thousands of problems. However, I don't have a good way of 
keeping track of my solutions and algorithms templates I use.   
This is a CLI based file manager with functionality based around my workflow. Stuff like storing solutions by category like dp, graph, number theory, etc.   
Finding solutions by name, key words, or catagory.   
Retrieving algorithms I use such as finding my implementation of network flow to use in a problem I'm working on.   


### Functionality
	- Be able to store solutions categorized 
		- Locate by name
		- Locate by categories
		- Keep track of some meta data (date, …other stuff??)
		- Be able to change categories around 
	- Be able to search for algorithms eg. Find the bipartite matching algorithm 
		- Not name sensitive since I’m bad at spelling (use some string algorithm for best matches)
		- Be able to add new algorithms by name, category, etc.
	- Multi threaded ??
	- Obviously store this data on disk, but a database is too heavy I prefer lightweight
	- Directory management (some files may not be in the same directory) 
	  - If a file is deleted in the disk then delete in the filesystem 
	  - So in general have the fs be updated periodically (Through my usage most files don't change too often)
	  - Keep a directory tree, and be able to change nodes. For example if I want to move a directory elsewhere but keep it in my system


### Specifications
    - No database usage since I dont' want to large software overhead
    - Keep directory data structure 
        - When adding files augment that global datastructure 
        - Be able to change nodes in that directory tree
    - Store problem data such as (solution, category list, key words, problem name)
    - Be able to retrieve and change problem data
    - Store all data in disk (deal with crashing maybe)
