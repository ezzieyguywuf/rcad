geom.rs
=======
Remove Scalar
-------------
It specifically exists to make multiplying Vector by a scalar value "easier" -
really, I just couldn't find a way to do `impl<T> Mul<Vector<T>> for T`, and
using it makes it possible.

Maybe Don't Remove Scalar
-------------------------
Maybe don't get rid of Scalar, and instead use it place of T for `Point` and
`Vector`. Maybe this way we can skirt around floating-point math limitations, or
at least delay them until as late as possible in our calculations.

Another idea is to define a `struct Unit(usize)`. The idea here is that a "unit"
is the smallest possible physical representation allowed in the CAD model - all
data structures and calculations can be based on the "unit", and ultimately at the
very very very end of the pipeline, we can apply a multiplier to make it more
easily understood.

For example, you could say that "unit" is equal to 1 mm. This obvs means that
1,000 "unit"s constitute a meter, but our algorithms don't have to know that.
Instead, they just do math as usual, and then at the end of the day, the main
loop takes the multipiler and applies it as necessary to display the user's
requested units (be they inches or meters or w/e).

The main drawback I see here is that we'd be asking the user to specify an
appropriate meaning for "unit" - like, what if someone wanted to model things at
the nanometer level all the way up to meters?

sure this would be a super details model by why the heck not?

Except what's the tradeoff? Will it become incredibly onerous to carry around so
many "unit"s of data for the larger things? Will it be a memory hog?
