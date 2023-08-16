# Things to start with

Render a little guy that can jump around

# Modelling the agent

Why spend time on this?

It's going to be the bedrock of the game, the most fun part, and the most challenging part.

## What is an agent?

### 1. Body (Input)

The first step of a level will be for the user to construct their little guy. To start, we'll restrict the body of the little guy to be just a simple circle.

Then, they'll add a series of senses. To start, we'll just focus on:

- Eyes. Each eye added gets an angle, and it will be able to monitor input in a region in front of it. To start, assume there is only one eye.
- OnGround (bool)

In the future we could add other senses (vision, temperature) or other body upgrades (spiky things to kill enemies, various speeds).

### 2. Brain

Note, this must be constructed _after_ all the body parts have been added. Each sense will correspond with _some_ number of inputs. I.e., in the general case, we'd probably want to have eyes have angular stuff, and maybe count enemies.

To start, eyes will track two things:

- What is the closest thing in my sight? (can be null) Will be a categorical variable, and will require every entity to have some kind of unique number/string
- How close is it

### 3. Brain (Output)

Then we'll have output tied to body parts. To start, we'll just have feet (movement). Output will be:

- Horizontal (-1.0 to 1.0)
- Jump (0.0 to 1.0)

Then
