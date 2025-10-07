# THREE.JS INTEGRATION ANALYSE

**Datum:** 2025-10-07  
**Zweck:** Analyse der 3D-Elemente aus dem Login-HTML für Svelte-Portierung

---

## 🎨 3D-ELEMENTE IM LOGIN-HTML

### Übersicht
Das Login-HTML verwendet **Three.js 0.160.0** für einen komplexen 3D-Hintergrund mit:
1. **Glow Gradient Background Sphere** (Shader-basiert)
2. **Particle Field** mit 4500 Partikeln (Shader-basiert)
3. **Wireframe Waves Plane** (animierte Geometrie)
4. **Mouse Parallax** (interaktive Kamera-Bewegung)

---

## 📦 KOMPONENTEN-ANALYSE

### 1. Glow Gradient Background Sphere
**Technologie:** Custom ShaderMaterial  
**Geometrie:** SphereGeometry(60, 64, 64)  
**Features:**
- Zeitbasierte Animation (uTime uniform)
- Noise-Funktion für organische Bewegung
- Farbmischung zwischen --bg und --accent
- BackSide rendering (Innenseite der Sphere)

**Shader-Code:**
```glsl
// Vertex Shader
varying vec3 vPos;
void main(){
  vPos = position;
  gl_Position = projectionMatrix*modelViewMatrix*vec4(position,1.0);
}

// Fragment Shader
varying vec3 vPos;
uniform float uTime;
uniform vec3 uColorA;
uniform vec3 uColorB;
uniform vec3 uAccent;

float noise(vec3 p){
  return fract(sin(dot(p, vec3(12.9898,78.233, 37.719)))*43758.5453);
}

void main(){
  vec3 dir = normalize(vPos);
  float n = noise(dir*4.0 + uTime*0.05);
  float g = smoothstep(-0.3, 0.8, dir.z + n*0.25);
  vec3 col = mix(uColorA, uColorB, g);
  col += uAccent * 0.05 * (0.5 + n);
  gl_FragColor = vec4(col, 1.0);
}
```

**Komplexität:** HOCH (Custom Shaders)

---

### 2. Particle Field (4500 Partikel)
**Technologie:** Points + Custom ShaderMaterial  
**Anzahl:** 4500 Partikel  
**Features:**
- Individuelle Geschwindigkeiten (aSpeed attribute)
- Sinusförmige Bewegung (price-like motion)
- Additive Blending (Glow-Effekt)
- Dynamische Punkt-Größe
- Farbe aus CSS-Variable (--brand)

**Shader-Code:**
```glsl
// Vertex Shader
attribute float aSpeed;
varying float vA;
uniform float uTime;

void main(){
  vec3 p = position;
  float t=uTime*0.08*aSpeed;
  p.y += sin(p.x*0.08 + t)*0.8 + sin(p.z*0.05 + t*1.2)*0.6;
  p.x += sin(p.y*0.05 + t*0.6)*0.4;
  vA = 0.6 + 0.4*sin(t + position.x*0.02);
  gl_PointSize = 1.8 + 1.6*vA;
  gl_Position = projectionMatrix*modelViewMatrix*vec4(p,1.0);
}

// Fragment Shader
varying float vA;
uniform vec3 uBrand;

void main(){
  float d = length(gl_PointCoord - vec2(0.5));
  float alpha = smoothstep(0.55, 0.0, d) * vA;
  gl_FragColor = vec4(uBrand, alpha);
}
```

**Komplexität:** SEHR HOCH (Custom Shaders + 4500 Partikel)

---

### 3. Wireframe Waves Plane
**Technologie:** LineSegments + WireframeGeometry  
**Geometrie:** PlaneGeometry(160, 100, 120, 80)  
**Features:**
- Animierte Z-Displacement (Wellen-Effekt)
- Sinusförmige Bewegung
- Farbe aus CSS-Variable (--accent)
- Transparenz (opacity: 0.28)

**Animation-Code:**
```javascript
const p = wf.geometry.attributes.position;
for(let i=0; i<p.count; i+=1){
  const x = p.getX(i), y = p.getY(i);
  const z = Math.sin((x*0.12 + t*0.6)) * Math.cos((y*0.18 + t*0.5)) * 1.2;
  p.setZ(i, z);
}
p.needsUpdate = true;
```

**Komplexität:** MITTEL (Geometrie-Manipulation)

---

### 4. Mouse Parallax
**Technologie:** Event Listener + Camera Movement  
**Features:**
- Smooth camera movement (lerp mit 0.03 factor)
- Responsive zu Maus-Position
- Normalisierte Koordinaten

**Code:**
```javascript
const target = new THREE.Vector2();
addEventListener('pointermove', (e)=>{
  target.x = (e.clientX / innerWidth - 0.5) * 0.8;
  target.y = (e.clientY / innerHeight - 0.5) * 0.6;
});

// In Animation Loop:
camera.position.x += (target.x*24 - camera.position.x) * 0.03;
camera.position.y += (target.y*10 - camera.position.y) * 0.03;
camera.lookAt(0,0,0);
```

**Komplexität:** NIEDRIG

---

## 🔧 SVELTE-PORTIERUNG: OPTIONEN

### Option 1: @threlte/core (EMPFOHLEN)
**Vorteile:**
- ✅ Native Svelte Integration
- ✅ Reaktive Three.js Komponenten
- ✅ TypeScript Support
- ✅ Gute Dokumentation
- ✅ Aktiv maintained

**Nachteile:**
- ⚠️ Zusätzliche Abstraktionsschicht
- ⚠️ Custom Shaders erfordern spezielle Syntax

**Installation:**
```bash
npm install three @threlte/core @threlte/extras
```

**Beispiel-Komponente:**
```svelte
<script lang="ts">
  import { Canvas } from '@threlte/core'
  import Scene from './Scene.svelte'
</script>

<Canvas>
  <Scene />
</Canvas>
```

---

### Option 2: svelte-cubed
**Vorteile:**
- ✅ Leichtgewichtig
- ✅ Einfache API

**Nachteile:**
- ❌ Weniger Features als @threlte
- ❌ Weniger aktiv maintained

**NICHT EMPFOHLEN**

---

### Option 3: Vanilla Three.js in Svelte
**Vorteile:**
- ✅ Volle Kontrolle
- ✅ Direkte Portierung möglich

**Nachteile:**
- ❌ Kein reaktives Binding
- ❌ Mehr Boilerplate
- ❌ Lifecycle-Management manuell

**Beispiel:**
```svelte
<script lang="ts">
  import { onMount } from 'svelte'
  import * as THREE from 'three'
  
  let canvas: HTMLCanvasElement
  
  onMount(() => {
    const renderer = new THREE.WebGLRenderer({ canvas, antialias: true })
    // ... rest of setup
    
    return () => {
      renderer.dispose()
    }
  })
</script>

<canvas bind:this={canvas} />
```

**MÖGLICH, aber @threlte ist besser**

---

## 📋 PORTIERUNGS-PLAN

### Phase 1: Setup
1. ✅ @threlte/core installieren
2. ✅ Three.js installieren
3. ✅ TypeScript Types installieren

### Phase 2: Basis-Komponenten
1. **BackgroundScene.svelte** - Haupt-Canvas
2. **GlowSphere.svelte** - Background Sphere mit Shader
3. **ParticleField.svelte** - 4500 Partikel mit Shader
4. **WireframePlane.svelte** - Animierte Wireframe-Ebene
5. **CameraController.svelte** - Mouse Parallax

### Phase 3: Shader-Integration
1. Custom ShaderMaterial in @threlte
2. Uniforms reaktiv machen (Svelte Stores)
3. Animation Loop mit @threlte's useFrame

### Phase 4: Performance-Optimierung
1. Partikel-Anzahl reduzieren für Mobile (responsive)
2. Shader-Komplexität reduzieren für Low-End Devices
3. Conditional Rendering (nur auf Desktop?)

---

## ⚡ PERFORMANCE-ÜBERLEGUNGEN

### Aktuelle Performance (HTML):
- **4500 Partikel** - GPU-intensiv
- **Custom Shaders** - GPU-intensiv
- **Animierte Geometrie** - CPU-intensiv (9600 Vertices)

### Optimierungen für Svelte:
1. **Responsive Partikel-Anzahl:**
   - Desktop: 4500
   - Tablet: 2000
   - Mobile: 500 oder deaktiviert

2. **Shader-Vereinfachung:**
   - Noise-Funktion optimieren
   - Weniger Berechnungen pro Frame

3. **Conditional Rendering:**
   ```svelte
   {#if !isMobile}
     <BackgroundScene />
   {:else}
     <StaticGradient />
   {/if}
   ```

4. **Lazy Loading:**
   - Three.js nur laden wenn benötigt
   - Code-Splitting

---

## 🎯 EMPFOHLENE IMPLEMENTIERUNG

### Datei-Struktur:
```
src/lib/components/3d/
├── BackgroundScene.svelte       # Haupt-Komponente
├── GlowSphere.svelte            # Shader-Sphere
├── ParticleField.svelte         # Partikel-System
├── WireframePlane.svelte        # Wireframe-Ebene
├── CameraController.svelte      # Mouse Parallax
└── shaders/
    ├── glowSphere.vert.glsl     # Vertex Shader
    ├── glowSphere.frag.glsl     # Fragment Shader
    ├── particles.vert.glsl      # Vertex Shader
    └── particles.frag.glsl      # Fragment Shader
```

### Verwendung:
```svelte
<!-- routes/login/+page.svelte -->
<script lang="ts">
  import BackgroundScene from '$lib/components/3d/BackgroundScene.svelte'
</script>

<div class="relative min-h-screen">
  <BackgroundScene />
  <div class="relative z-10">
    <!-- Login Form -->
  </div>
</div>
```

---

## 📊 AUFWANDS-SCHÄTZUNG

| Komponente | Komplexität | Zeitaufwand |
|-----------|-------------|-------------|
| Setup (@threlte) | Niedrig | 1h |
| GlowSphere | Hoch | 4h |
| ParticleField | Sehr Hoch | 6h |
| WireframePlane | Mittel | 3h |
| CameraController | Niedrig | 1h |
| Performance-Optimierung | Mittel | 3h |
| Testing & Debugging | - | 4h |
| **GESAMT** | - | **22h** |

---

## ✅ NÄCHSTE SCHRITTE

1. ✅ Entscheidung: @threlte/core verwenden
2. ⏭️ Package installieren
3. ⏭️ Basis-Komponente erstellen
4. ⏭️ Shader portieren
5. ⏭️ Performance testen
6. ⏭️ Mobile-Optimierung

---

## 🚨 WICHTIGE HINWEISE

1. **rebuild.md erwähnt Three.js NICHT** - Dies ist eine zusätzliche Anforderung!
2. **Performance-kritisch** - 4500 Partikel + Shaders sind GPU-intensiv
3. **Mobile-Support** - Muss responsive sein (evtl. deaktivieren auf Mobile)
4. **Bundle-Size** - Three.js ist ~600KB (minified) - Code-Splitting verwenden!

