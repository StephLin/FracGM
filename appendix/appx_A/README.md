# :page_facing_up: A simple example of FracGM with global optimal guarantees

We give a simple example that satisfies Proposition 3 as follows. 
Suppose we have an optimization problem:

$$
    \min_x\ \frac{f(x)}{h(x)}=\frac{x^2}{x^2+1},
$$

where the residual $r(x)=x$ and $c=1$ in sense of Geman-McClure. The dual problem $(6)$ is as follows:

$$
    \min_x\ \mu\big(f(x)-\beta h(x)\big)=\mu\big(x^2-\beta(x^2+1)\big)=\mu x^2-\mu\beta x^2-\mu\beta,
$$

where $\beta\in\mathbb{R}$ and $\mu\in\mathbb{R}$. Let $\boldsymbol{\alpha}=(\beta,\mu)^\top\in\mathbb{R}^2$ and $x_{\boldsymbol{\alpha}}$ be the optimal solution of Problem $(6)$ given $\boldsymbol{\alpha}$, we obtain $x_{\boldsymbol{\alpha}}=0$ by setting the derivative to zero as follows:

$$
    \frac{\partial}{\partial x}(\mu x^2-\mu\beta x^2-\mu\beta)=2\mu(1-\beta)x=0\implies x^*=x_{\boldsymbol{\alpha}}=0.
$$

Thereby we can write 

$$
    \psi(\boldsymbol{\alpha},x_{\boldsymbol{\alpha}})=
    \begin{pmatrix}
    -f(x_{\boldsymbol{\alpha}})+\beta h(x_{\boldsymbol{\alpha}})\\
    -1+\mu h(x_{\boldsymbol{\alpha}})
    \end{pmatrix}=
    \begin{pmatrix}
    -x_{\boldsymbol{\alpha}}^2+\beta x_{\boldsymbol{\alpha}}^2+\beta\\
    -1+\mu x_{\boldsymbol{\alpha}}^2+\mu
    \end{pmatrix}=
    \begin{pmatrix}
    \beta\\
    \mu-1
    \end{pmatrix}.
$$

It is trivial that both component functions $\psi_1(\boldsymbol{\alpha},x_{\boldsymbol{\alpha}})=\beta$ and $\psi_2(\boldsymbol{\alpha},x_{\boldsymbol{\alpha}})=\mu-1$ are differentiable, thus $\psi(\boldsymbol{\alpha},x_{\boldsymbol{\alpha}})$ is differentiable. Given $\boldsymbol{\alpha}_1=(\beta_1,\mu_1)^\top$ and $\boldsymbol{\alpha}_2=(\beta_2,\mu_2)^\top$,

$$
    \\|\psi(\boldsymbol{\alpha}\_1,x_{\boldsymbol{\alpha}\_1})-\psi(\boldsymbol{\alpha}\_2,x_{\boldsymbol{\alpha}_2})\\|=
    \begin{Vmatrix}
    \beta_1-\beta_2\\\\\mu_1-1-(\mu_2-1)
    \end{Vmatrix}\leq\begin{Vmatrix}
    \beta_1-\beta_2\\\\\mu_1-\mu_2
    \end{Vmatrix},
$$

then $\psi(\boldsymbol{\alpha},x_{\boldsymbol{\alpha}})$ is Lipschitz continuous. Solving such (simple) case by FracGM guarantees that the solution is global optimal.


To verify the above statement empirically, we feed various initial guesses to FracGM to examine the global optimality as follows:

| Initial Guess | FracGM's 1$^\text{st}$ Iteration | FracGM's 2$^\text{nd}$ Iteration |
|---------------|----------------------------------|----------------------------------|
| $-10^{5}$     | $-2.20\times 10^{-4}$            | $0.00\times 10^{-13}$            |
| $-10^{3}$     | $-1.92\times 10^{-10}$           | $0.00\times 10^{-13}$            |
| $-10^{1}$     | $-5.10\times 10^{-8}$            | $0.00\times 10^{-13}$            |
| $-10^{0}$     | $-2.73\times 10^{-9}$            | $0.00\times 10^{-13}$            |
| $10^{0}$      | $-2.73\times 10^{-9}$            | $0.00\times 10^{-13}$            |
| $10^{1}$      | $-5.10\times 10^{-8}$            | $0.00\times 10^{-13}$            |
| $10^{3}$      | $-1.92\times 10^{-10}$           | $0.00\times 10^{-13}$            |
| $10^{5}$      | $-2.20\times 10^{-4}$            | $0.00\times 10^{-13}$            |


## :running: Run
```
cd appendix/appx_A
python ./main.py
```

We test various random initial guess between -100000 and 100000, all solutions converge to 0.

```
=========================
initial guess: -90760
solution: 0.0
=========================
initial guess: 90371
solution: 0.0
=========================
initial guess: 71362
solution: 0.0
=========================
initial guess: 25425
solution: 0.0
=========================
initial guess: -82805
solution: 0.0
=========================
initial guess: 42735
solution: 0.0
=========================
initial guess: -174
solution: 0.0
=========================
initial guess: -12185
solution: 0.0
=========================
initial guess: 33929
solution: 0.0
=========================
initial guess: -23746
solution: 0.0
```