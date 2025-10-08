<script lang="ts">
  import { goto } from '$app/navigation';
  import { isAuthenticated, authStore } from '$lib/stores/auth';
  import { authApi, ApiError } from '$lib/api/client';
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';

  let activeTab: 'login' | 'register' = 'login';
  let loginEmail = '';
  let loginPassword = '';
  let registerName = '';
  let registerEmail = '';
  let registerPassword = '';
  let rememberMe = true;
  let agreeTerms = false;
  let passwordStrength = 0;
  let canvas: HTMLCanvasElement;
  let loginError = '';
  let registerError = '';
  let isLoading = false;

  onMount(() => {
    // Redirect to dashboard if already logged in
    if ($isAuthenticated) {
      goto('/dashboard');
    }

    // Initialize 3D background
    if (browser && canvas) {
      init3DBackground();
    }
  });

  function init3DBackground() {
    // @ts-ignore - THREE is loaded from CDN
    if (typeof THREE === 'undefined') {
      const script = document.createElement('script');
      script.src = 'https://cdn.jsdelivr.net/npm/three@0.160.0/build/three.min.js';
      script.onload = () => setup3D();
      document.head.appendChild(script);
    } else {
      setup3D();
    }
  }

  function setup3D() {
    // @ts-ignore
    const THREE = window.THREE;
    const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(55, window.innerWidth / window.innerHeight, 0.1, 2000);
    camera.position.set(0, 0, 42);

    function resize() {
      const dpr = Math.min(2, window.devicePixelRatio || 1);
      renderer.setPixelRatio(dpr);
      renderer.setSize(window.innerWidth, window.innerHeight);
      camera.aspect = window.innerWidth / window.innerHeight;
      camera.updateProjectionMatrix();
    }
    window.addEventListener('resize', resize);
    resize();

    // Glow gradient background sphere
    const glowGeo = new THREE.SphereGeometry(60, 64, 64);
    const glowMat = new THREE.ShaderMaterial({
      uniforms: {
        uTime: { value: 0 },
        uColorA: { value: new THREE.Color('#0d1b2a') },
        uColorB: { value: new THREE.Color('#0d1b2a') },
        uAccent: { value: new THREE.Color('#7cfc00') }
      },
      vertexShader: `
        varying vec3 vPos;
        void main() {
          vPos = position;
          gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
        }
      `,
      fragmentShader: `
        varying vec3 vPos;
        uniform float uTime;
        uniform vec3 uColorA;
        uniform vec3 uColorB;
        uniform vec3 uAccent;
        float noise(vec3 p) {
          return fract(sin(dot(p, vec3(12.9898, 78.233, 37.719))) * 43758.5453);
        }
        void main() {
          vec3 dir = normalize(vPos);
          float n = noise(dir * 4.0 + uTime * 0.05);
          float g = smoothstep(-0.3, 0.8, dir.z + n * 0.25);
          vec3 col = mix(uColorA, uColorB, g);
          col += uAccent * 0.05 * (0.5 + n);
          gl_FragColor = vec4(col, 1.0);
        }
      `,
      side: THREE.BackSide
    });
    const glow = new THREE.Mesh(glowGeo, glowMat);
    scene.add(glow);

    // Particle field
    const COUNT = 4500;
    const g = new THREE.BufferGeometry();
    const pos = new Float32Array(COUNT * 3);
    const spd = new Float32Array(COUNT);
    for (let i = 0; i < COUNT; i++) {
      pos[i * 3 + 0] = (Math.random() - 0.5) * 120;
      pos[i * 3 + 1] = (Math.random() - 0.5) * 60;
      pos[i * 3 + 2] = (Math.random() - 0.5) * 40;
      spd[i] = 0.5 + Math.random() * 1.5;
    }
    g.setAttribute('position', new THREE.BufferAttribute(pos, 3));
    g.setAttribute('aSpeed', new THREE.BufferAttribute(spd, 1));

    const pMat = new THREE.ShaderMaterial({
      transparent: true,
      depthWrite: false,
      blending: THREE.AdditiveBlending,
      uniforms: {
        uTime: { value: 0 },
        uBrand: { value: new THREE.Color('#1b9aaa') }
      },
      vertexShader: `
        attribute float aSpeed;
        varying float vA;
        uniform float uTime;
        void main() {
          vec3 p = position;
          float t = uTime * 0.08 * aSpeed;
          p.y += sin(p.x * 0.08 + t) * 0.8 + sin(p.z * 0.05 + t * 1.2) * 0.6;
          p.x += sin(p.y * 0.05 + t * 0.6) * 0.4;
          vA = 0.6 + 0.4 * sin(t + position.x * 0.02);
          gl_PointSize = 1.8 + 1.6 * vA;
          gl_Position = projectionMatrix * modelViewMatrix * vec4(p, 1.0);
        }
      `,
      fragmentShader: `
        varying float vA;
        uniform vec3 uBrand;
        void main() {
          float d = length(gl_PointCoord - vec2(0.5));
          float alpha = smoothstep(0.55, 0.0, d) * vA;
          gl_FragColor = vec4(uBrand, alpha);
        }
      `
    });
    const points = new THREE.Points(g, pMat);
    scene.add(points);


    // Wireframe waves plane
    const planeGeo = new THREE.PlaneGeometry(160, 100, 120, 80);
    const planeMat = new THREE.LineBasicMaterial({
      color: new THREE.Color('#7cfc00'),
      transparent: true,
      opacity: 0.28
    });
    const wf = new THREE.LineSegments(new THREE.WireframeGeometry(planeGeo), planeMat);
    wf.rotation.x = -Math.PI / 3;
    wf.position.set(0, -20, -10);
    scene.add(wf);

    // Mouse parallax
    const target = new THREE.Vector2();
    window.addEventListener('pointermove', (e) => {
      target.x = (e.clientX / window.innerWidth - 0.5) * 0.8;
      target.y = (e.clientY / window.innerHeight - 0.5) * 0.6;
    });

    let t = 0;
    function animate() {
      requestAnimationFrame(animate);
      t += 1 / 60;
      glowMat.uniforms.uTime.value = t;
      pMat.uniforms.uTime.value = t;
      camera.position.x += (target.x * 24 - camera.position.x) * 0.03;
      camera.position.y += (target.y * 10 - camera.position.y) * 0.03;
      camera.lookAt(0, 0, 0);

      // Animate wireframe Z displacement
      const p = wf.geometry.attributes.position;
      for (let i = 0; i < p.count; i += 1) {
        const x = p.getX(i),
          y = p.getY(i);
        const z = Math.sin(x * 0.12 + t * 0.6) * Math.cos(y * 0.18 + t * 0.5) * 1.2;
        p.setZ(i, z);
      }
      p.needsUpdate = true;
      renderer.render(scene, camera);
    }
    animate();
  }

  function calculatePasswordStrength(password: string): number {
    let strength = 0;
    if (password.length >= 8) strength++;
    if (/[A-Z]/.test(password)) strength++;
    if (/[0-9]/.test(password)) strength++;
    if (/[^\w]/.test(password)) strength++;
    return strength;
  }

  function handlePasswordInput(e: Event) {
    const target = e.target as HTMLInputElement;
    passwordStrength = calculatePasswordStrength(target.value);
  }

  async function handleLogin(e: Event) {
    e.preventDefault();
    loginError = '';
    isLoading = true;

    try {
      const response = await authApi.login(loginEmail, loginPassword);
      authStore.setAuth(response.user, response.token);
      goto('/dashboard');
    } catch (err) {
      if (err instanceof ApiError) {
        loginError = err.details || err.message;
      } else {
        loginError = 'An unexpected error occurred';
      }
    } finally {
      isLoading = false;
    }
  }

  async function handleRegister(e: Event) {
    e.preventDefault();
    registerError = '';

    if (passwordStrength < 3) {
      registerError = 'Please use a stronger password (at least 8 characters with uppercase, number, and symbol)';
      return;
    }

    if (!agreeTerms) {
      registerError = 'Please agree to the terms and privacy policy';
      return;
    }

    isLoading = true;

    try {
      const response = await authApi.register(registerName, registerEmail, registerPassword);
      authStore.setAuth(response.user, response.token);
      goto('/dashboard');
    } catch (err) {
      if (err instanceof ApiError) {
        registerError = err.details || err.message;
      } else {
        registerError = 'An unexpected error occurred';
      }
    } finally {
      isLoading = false;
    }
  }

  $: strengthLabel = ['Too weak', 'Weak', 'Okay', 'Strong', 'Excellent'][passwordStrength] || 'Too weak';
</script>

<svelte:head>
  <title>PriceActionTalk — Sign in / Create account</title>
  <meta name="description" content="Dark 3D one‑pager: login & register for the PriceActionTalk trading journal tool." />
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&display=swap" rel="stylesheet" />
</svelte:head>

<canvas bind:this={canvas} id="bg3d" aria-hidden="true" class="fixed inset-0 -z-20"></canvas>
<div class="vignette" aria-hidden="true"></div>

<main class="wrap">
  <section class="card" role="region" aria-label="PriceActionTalk authentication">
    <!-- LEFT: Brand & Selling points -->
    <div class="hero">
      <div class="logo" aria-label="PriceActionTalk">
        <span class="dot" aria-hidden="true"></span>
        <span>PriceActionTalk</span>
      </div>
      <h1>Elevate your trading journal.</h1>
      <p>One secure account for journaling, strategy building, and performance analytics. Join a community of disciplined market operators.</p>

      <div class="features">
        <div class="chip">✨ Integrated Trading Journal</div>
        <div class="chip">⏲️ Session & risk tracking</div>
        <div class="chip">📈 Strategy builder</div>
        <div class="chip">👁️ Pattern tagging</div>
        <div class="chip">⚡ Fast import from brokers</div>
        <div class="chip">👥 Community insights</div>
      </div>

      <div class="ticker" aria-hidden="true">
        <div class="line">
          <span class="badge"><span class="b"></span>EURUSD +0.42%</span>
          <span class="badge"><span class="b"></span>NAS100 +1.02%</span>
          <span class="badge"><span class="b"></span>BTC +2.80%</span>
          <span class="badge"><span class="b"></span>WTI -0.35%</span>
          <span class="badge"><span class="b"></span>XAU +0.12%</span>
          <span class="badge"><span class="b"></span>US10Y 3.95%</span>
          &nbsp;&nbsp;&nbsp;•&nbsp;&nbsp;&nbsp;repeat…
        </div>
      </div>
    </div>

    <!-- RIGHT: Auth -->
    <div class="auth">
      <div class="tabs" role="tablist" aria-label="Authentication tabs">
        <button
          class="tab"
          class:active={activeTab === 'login'}
          role="tab"
          aria-selected={activeTab === 'login'}
          aria-controls="panel-login"
          id="tab-login"
          on:click={() => (activeTab = 'login')}
        >
          Sign in
        </button>
        <button
          class="tab"
          class:active={activeTab === 'register'}
          role="tab"
          aria-selected={activeTab === 'register'}
          aria-controls="panel-register"
          id="tab-register"
          on:click={() => (activeTab = 'register')}
        >
          Create account
        </button>
      </div>


      <!-- LOGIN -->
      {#if activeTab === 'login'}
        <form id="panel-login" role="tabpanel" aria-labelledby="tab-login" on:submit={handleLogin}>
          {#if loginError}
            <div class="error-message">{loginError}</div>
          {/if}
          <div class="group">
            <label for="lemail">Email</label>
            <div class="field">
              <input id="lemail" name="email" type="email" placeholder="you@domain.com" autocomplete="email" bind:value={loginEmail} required disabled={isLoading} />
            </div>
          </div>
          <div class="group">
            <label for="lpassword">Password</label>
            <div class="field">
              <input id="lpassword" name="password" type="password" placeholder="••••••••" autocomplete="current-password" bind:value={loginPassword} required disabled={isLoading} />
            </div>
            <div class="row">
              <label class="checkbox">
                <input id="remember" type="checkbox" bind:checked={rememberMe} disabled={isLoading} />
                Remember me
              </label>
              <a href="#" class="hint">Forgot password?</a>
            </div>
          </div>
          <button class="btn primary" type="submit" disabled={isLoading}>
            {isLoading ? 'Signing in...' : 'Sign in'}
          </button>
          <div class="divider">or continue with</div>
          <div class="socials">
            <a class="btn ghost" href="#" aria-label="Continue with Google">● Google</a>
            <a class="btn ghost" href="#" aria-label="Continue with GitHub">💻 GitHub</a>
          </div>
        </form>
      {/if}

      <!-- REGISTER -->
      {#if activeTab === 'register'}
        <form id="panel-register" role="tabpanel" aria-labelledby="tab-register" on:submit={handleRegister}>
          {#if registerError}
            <div class="error-message">{registerError}</div>
          {/if}
          <div class="group">
            <label for="rname">Full name</label>
            <div class="field">
              <input id="rname" name="name" type="text" placeholder="First Last" autocomplete="name" bind:value={registerName} required disabled={isLoading} />
            </div>
          </div>
          <div class="group">
            <label for="remail">Email</label>
            <div class="field">
              <input id="remail" name="email" type="email" placeholder="you@domain.com" autocomplete="email" bind:value={registerEmail} required disabled={isLoading} />
            </div>
          </div>
          <div class="group">
            <label for="rpassword">Password</label>
            <div class="field">
              <input
                id="rpassword"
                name="password"
                type="password"
                placeholder="8+ characters"
                autocomplete="new-password"
                bind:value={registerPassword}
                on:input={(e) => (passwordStrength = calculatePasswordStrength(e.currentTarget.value))}
                required
                disabled={isLoading}
              />
            </div>
            <div class="strength" aria-hidden="true">
              <div class="bar" style="width: {(passwordStrength / 4) * 100}%"></div>
            </div>
            <span class="hint">{strengthLabel}</span>
          </div>
          <div class="row">
            <label class="checkbox">
              <input id="terms" type="checkbox" bind:checked={agreeTerms} required disabled={isLoading} />
              I agree to the <a href="#" class="hint">Terms</a> & <a href="#" class="hint">Privacy</a>
            </label>
          </div>
          <button class="btn primary" type="submit" disabled={isLoading}>
            {isLoading ? 'Creating account...' : 'Create account'}
          </button>
          <div class="divider">or sign up with</div>
          <div class="socials">
            <a class="btn ghost" href="#" aria-label="Continue with Google">● Google</a>
            <a class="btn ghost" href="#" aria-label="Continue with GitHub">💻 GitHub</a>
          </div>
        </form>
      {/if}
    </div>
  </section>
</main>

<footer class="footer">© 2025 PriceActionTalk — Trading Journal & Strategy Builder</footer>

<style>
  :global(body) {
    margin: 0;
    font-family: Inter, system-ui, -apple-system, 'Segoe UI', Roboto, Ubuntu, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif;
    color: #ffffff;
    background: radial-gradient(1200px 800px at 10% 10%, rgba(27, 154, 170, 0.2) 0%, #0d1b2a 60%), #0d1b2a;
    overflow-x: hidden;
  }

  #bg3d {
    position: fixed;
    inset: 0;
    z-index: -2;
  }

  .vignette {
    position: fixed;
    inset: -10px;
    pointer-events: none;
    z-index: -1;
    background: radial-gradient(1200px 800px at 85% 20%, rgba(27, 154, 170, 0.2) 0%, rgba(27, 154, 170, 0) 60%),
      radial-gradient(900px 600px at 20% 80%, rgba(124, 252, 0, 0.18) 0%, rgba(124, 252, 0, 0) 60%),
      radial-gradient(1200px 900px at 50% 50%, rgba(0, 0, 0, 0.35) 40%, rgba(0, 0, 0, 0.65) 100%);
    filter: saturate(1.2);
  }

  .wrap {
    position: relative;
    min-height: 100vh;
    display: grid;
    place-items: center;
    padding: clamp(16px, 3vw, 32px);
  }

  .card {
    width: min(980px, 94vw);
    backdrop-filter: blur(16px) saturate(120%);
    background: linear-gradient(180deg, rgba(46, 46, 46, 0.55) 0%, rgba(46, 46, 46, 0.85) 100%);
    border: 1px solid rgba(27, 154, 170, 0.22);
    border-radius: 24px;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.55);
    overflow: hidden;
    display: grid;
    grid-template-columns: 1.1fr 1fr;
  }

  @media (max-width: 980px) {
    .card {
      grid-template-columns: 1fr;
    }
  }

  .hero {
    position: relative;
    padding: 40px 40px 24px;
    overflow: hidden;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 12px;
    font-weight: 800;
    letter-spacing: 0.2px;
    text-transform: uppercase;
  }

  .dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: linear-gradient(135deg, #1b9aaa, #7cfc00);
    box-shadow: 0 0 24px #1b9aaa;
  }

  .hero h1 {
    margin: 18px 0 12px;
    font-size: clamp(28px, 3.4vw, 40px);
    line-height: 1.1;
  }

  .hero p {
    margin: 0;
    color: #cfd6dd;
    font-size: clamp(14px, 1.4vw, 16px);
  }


  .features {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 12px;
    margin-top: 24px;
  }

  .chip {
    border: 1px dashed rgba(124, 252, 0, 0.35);
    background: rgba(124, 252, 0, 0.1);
    padding: 10px 12px;
    border-radius: 12px;
    font-size: 13px;
    color: #ebf4ff;
  }

  @media (max-width: 980px) {
    .features {
      grid-template-columns: 1fr 1fr;
    }
  }

  .auth {
    position: relative;
    padding: 28px;
    background: linear-gradient(180deg, rgba(13, 27, 42, 0.35), rgba(13, 27, 42, 0.75));
    border-left: 1px solid rgba(255, 255, 255, 0.06);
  }

  .tabs {
    display: flex;
    gap: 8px;
    background: rgba(255, 255, 255, 0.06);
    padding: 6px;
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .tab {
    flex: 1;
    text-align: center;
    cursor: pointer;
    padding: 10px 12px;
    border-radius: 10px;
    font-weight: 700;
    color: #eaf5ff;
    transition: all 0.25s ease;
    background: transparent;
    border: none;
  }

  .tab.active {
    background: linear-gradient(180deg, rgba(27, 154, 170, 0.18), rgba(124, 252, 0, 0.14));
    color: white;
    outline: 1px solid rgba(27, 154, 170, 0.35);
  }

  form {
    margin-top: 18px;
    display: grid;
    gap: 14px;
  }

  label {
    font-size: 12px;
    color: #dce6ef;
    display: block;
    margin-bottom: 6px;
  }

  .group {
    display: grid;
    gap: 8px;
  }

  .field {
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 14px;
    padding: 10px 12px;
    outline: 0;
    transition:
      border-color 0.2s,
      box-shadow 0.2s;
  }

  .field:has(input:focus) {
    box-shadow: 0 0 0 4px rgba(27, 154, 170, 0.45);
    border-color: rgba(27, 154, 170, 0.55);
  }

  .field input {
    flex: 1;
    background: transparent;
    border: 0;
    outline: 0;
    color: #ffffff;
    font-size: 15px;
    padding: 6px 2px;
  }

  .hint {
    font-size: 12px;
    color: #c6d3dd;
  }

  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .checkbox {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 13px;
    color: #e6f1f7;
  }

  .checkbox input {
    width: 16px;
    height: 16px;
    accent-color: #1b9aaa;
  }

  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 12px;
    padding: 12px 14px;
    cursor: pointer;
    font-weight: 800;
    transition:
      transform 0.1s ease,
      box-shadow 0.25s ease,
      background 0.25s ease;
    text-decoration: none;
  }

  .btn:active {
    transform: translateY(1px);
  }

  .btn.primary {
    background: linear-gradient(135deg, rgba(27, 154, 170, 0.22), rgba(124, 252, 0, 0.18));
    color: white;
    box-shadow:
      0 10px 30px rgba(27, 154, 170, 0.2),
      inset 0 0 24px rgba(124, 252, 0, 0.25);
  }

  .btn.primary:hover {
    box-shadow:
      0 14px 40px rgba(27, 154, 170, 0.28),
      inset 0 0 30px rgba(124, 252, 0, 0.35);
  }

  .divider {
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    gap: 12px;
    color: #b9c7d2;
    font-size: 12px;
  }

  .divider::before,
  .divider::after {
    content: '';
    height: 1px;
    background: linear-gradient(90deg, rgba(255, 255, 255, 0), rgba(255, 255, 255, 0.22), rgba(255, 255, 255, 0));
  }


  .socials {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
  }

  .btn.ghost {
    background: rgba(255, 255, 255, 0.06);
    color: #ecf5ff;
  }

  .strength {
    height: 8px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 999px;
    overflow: hidden;
  }

  .bar {
    height: 100%;
    width: 0%;
    background: linear-gradient(90deg, #ef4444, #f59e0b, #7cfc00);
    transition: width 0.35s ease;
  }

  .ticker {
    position: absolute;
    bottom: -1px;
    left: -1px;
    right: -1px;
    height: 40px;
    background: linear-gradient(180deg, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.35));
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    overflow: hidden;
  }

  .ticker .line {
    position: absolute;
    white-space: nowrap;
    bottom: 8px;
    left: 0;
    right: 0;
    animation: ticker 30s linear infinite;
    opacity: 0.9;
  }

  @keyframes ticker {
    0% {
      transform: translateX(0);
    }
    100% {
      transform: translateX(-50%);
    }
  }

  .badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    border: 1px solid rgba(255, 255, 255, 0.14);
    background: rgba(255, 255, 255, 0.08);
    padding: 6px 10px;
    border-radius: 999px;
    margin-right: 12px;
    font-size: 12px;
  }

  .badge .b {
    display: inline-block;
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #1b9aaa;
    box-shadow: 0 0 12px #1b9aaa;
  }

  .footer {
    position: fixed;
    bottom: 12px;
    left: 0;
    right: 0;
    text-align: center;
    color: #8ea0ad;
    font-size: 12px;
  }

  .error-message {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #fca5a5;
    padding: 12px;
    border-radius: 12px;
    font-size: 13px;
    margin-bottom: 14px;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
