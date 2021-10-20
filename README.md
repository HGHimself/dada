# DADA
### Dadaist Poem Generator
https://en.wikipedia.org/wiki/Dada

## [Cut-Up Technique](https://en.wikipedia.org/wiki/Cut-up_technique)
```
TO MAKE A DADAIST POEM
Take a newspaper.
Take some scissors.
Choose from this paper an article of the length you want to make your poem.
Cut out the article.
Next carefully cut out each of the words that makes up this article and put them all in a bag.
Shake gently.
Next take out each cutting one after the other.
Copy conscientiously in the order in which they left the bag.
The poem will resemble you.
And there you are – an infinitely original author of charming sensibility, even though unappreciated by the vulgar herd.
```
## Usage
This is a function that will take a string of any length and convert it into a Dadaist poem. Essentially rearranging the words into a random order.

```rust
use dada_poem_generator::dada;

fn main() {
    let text = "TO MAKE A DADAIST POEM
Take a newspaper.
Take some scissors.
Choose from this paper an article of the length you want to make your poem.
Cut out the article.
Next carefully cut out each of the words that makes up this article and put them all in a bag.
Shake gently.
Next take out each cutting one after the other.
Copy conscientiously in the order in which they left the bag.
The poem will resemble you.
And there you are – an infinitely original author of charming sensibility, even though unappreciated by the vulgar herd.";

    println!("{}", dada(text));
/* OUTPUTS
bag charming want all article
left in newspaper.
the some you.
POEM Take though words cut in an each out carefully the a each will makes.
a put they the.
this infinitely after of vulgar by an Choose one of The order out which Next Next length cutting make them.
other Take.
And you resemble scissors poem the to paper this.
unappreciated even the of Copy the out and your in author.
you original there from DADAIST.
Shake that bag MAKE – sensibility up TO Cut conscientiously take poem, gently herd article are A the article.
*/
}
```
