<!doctype html>
<html lang="es">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width,initial-scale=1" />
  <title>Project_58 — Gamifica el cole</title>
  <style>
    :root{
      --bg:#0f1724; --card:#0b1220; --muted:#94a3b8; --accent:#8b5cf6; --glass: rgba(255,255,255,0.04);
      --glass-2: rgba(255,255,255,0.02);
    }
    *{box-sizing:border-box}
    body{margin:0;font-family:Inter, ui-sans-serif, system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial;background:linear-gradient(180deg,#071025 0%, #07132a 60%);color:#e6eef8;-webkit-font-smoothing:antialiased}
    .container{max-width:1100px;margin:48px auto;padding:28px}

    header{display:flex;gap:20px;align-items:center}
    .logo{width:120px;height:120px;border-radius:18px;background:linear-gradient(135deg, rgba(255,255,255,0.02), transparent);display:flex;align-items:center;justify-content:center;padding:12px;box-shadow:0 6px 18px rgba(0,0,0,0.6);border:1px solid rgba(255,255,255,0.03)}
    .logo img{max-width:100%;max-height:100%;object-fit:contain;border-radius:12px}

    h1{font-size:34px;margin:0}
    p.lead{color:var(--muted);margin-top:6px}

    .badges{display:flex;gap:8px;margin-top:12px}
    .badge{padding:6px 10px;border-radius:999px;background:var(--glass);color:var(--muted);font-size:13px;border:1px solid rgba(255,255,255,0.03)}

    main{display:grid;grid-template-columns:1fr 360px;gap:26px;margin-top:28px}

    .card{background:linear-gradient(180deg, rgba(255,255,255,0.02), rgba(255,255,255,0.01));border-radius:14px;padding:18px;border:1px solid rgba(255,255,255,0.03);box-shadow:0 8px 30px rgba(2,6,23,0.6)}

    .features{display:grid;grid-template-columns:repeat(2,1fr);gap:12px;margin-top:12px}
    .feature{background:var(--glass-2);padding:12px;border-radius:10px;border:1px solid rgba(255,255,255,0.02)}
    .feature h4{margin:0 0 6px 0}
    .feature p{margin:0;color:var(--muted);font-size:14px}

    .right .preview{height:260px;border-radius:12px;display:flex;align-items:center;justify-content:center;overflow:hidden;background:linear-gradient(120deg, rgba(139,92,246,0.12), rgba(59,130,246,0.06));border:1px solid rgba(255,255,255,0.03)}
    .right .preview img{max-width:100%;height:100%;object-fit:contain}

    .grid-cta{display:grid;grid-template-columns:repeat(3,1fr);gap:10px;margin-top:12px}
    .cta{padding:10px;border-radius:10px;text-align:center;background:var(--glass);border:1px solid rgba(255,255,255,0.02)}
    .cta strong{display:block;font-size:18px}
    .muted{color:var(--muted)}

    footer{margin-top:20px;color:var(--muted);font-size:13px}

    /* responsive */
    @media (max-width:900px){main{grid-template-columns:1fr} .right{order:-1}}
  </style>
</head>
<body>
  <div class="container">
    <header>
      <div class="logo card"><img src="assets/mascota.png" alt="Mascota Project_58"/></div>
      <div>
        <h1>Project_58 <span style="color:var(--accent);">— Gamifica el cole</span></h1>
        <p class="lead">Sistema para alentar a los estudiantes a asistir y participar en clase. Convertimos el cole en un videojuego con logros, personalización y economía interna.</p>
        <div class="badges">
          <div class="badge">🎯 Logros</div>
          <div class="badge">🧑‍🎨 Customización</div>
          <div class="badge">💰 Sistema de monedas</div>
        </div>
      </div>
    </header>

    <main>
      <section>
        <div class="card">
          <h2>¿Qué hace Project_58?</h2>
          <p class="muted">Project_58 transforma la asistencia y participación en clase en mecánicas de juego que motivan a los estudiantes. Implementamos:</p>

          <div class="features" style="margin-top:14px">
            <div class="feature">
              <h4>🏆 Sistema de Logros</h4>
              <p>Misiones diarias, retos semanales y medallas por asistencia y participación.</p>
            </div>
            <div class="feature">
              <h4>🧢 Personalización</h4>
              <p>Avatares y objetos desbloqueables para que cada estudiante arme su personaje.</p>
            </div>
            <div class="feature">
              <h4>💸 Economía interna</h4>
              <p>Monedas por participación que se usan para comprar items o ventajas dentro del sistema.</p>
            </div>
            <div class="feature">
              <h4>📊 Reportes</h4>
              <p>Paneles para profes y directivos con métricas de asistencia y engagement.</p>
            </div>
          </div>

          <h3 style="margin-top:18px">Cómo integrarlo</h3>
          <ol class="muted" style="padding-left:18px">
            <li>Sube los assets a <code>assets/</code> (logos, mascots, gifs).</li>
            <li>Incluye <code>README.html</code> o <code>index.html</code> en la raíz del repo para presentación.</li>
            <li>Documenta endpoints y cómo correr el servidor (Dockerfile recomendado).</li>
          </ol>

          <div style="margin-top:14px;display:flex;gap:12px;align-items:center">
            <a href="#" style="background:var(--accent);color:white;padding:10px 14px;border-radius:10px;text-decoration:none;font-weight:600">Abrir demo</a>
            <a href="#" style="padding:10px 14px;border-radius:10px;background:transparent;border:1px solid rgba(255,255,255,0.04);color:var(--muted);text-decoration:none">Ver guía</a>
          </div>
        </div>

        <div class="card" style="margin-top:14px">
          <h3>Preview de componentes</h3>
          <p class="muted">Ejemplos de piezas que puedes agregar al repo (docs, screenshots, gifs, banners).</p>
          <div style="display:flex;gap:10px;margin-top:12px;flex-wrap:wrap">
            <div style="flex:1;min-width:200px" class="cta card">
              <strong>README.md</strong>
              <span class="muted">Con imagen principal, badges y tabla de contenidos.</span>
            </div>
            <div style="flex:1;min-width:200px" class="cta card">
              <strong>Docs/</strong>
              <span class="muted">Sección con API y cómo desplegar.</span>
            </div>
            <div style="flex:1;min-width:200px" class="cta card">
              <strong>assets/</strong>
              <span class="muted">Icono, mascota, gifs demo.</span>
            </div>
          </div>
        </div>
      </section>

      <aside class="right">
        <div class="card">
          <div class="preview">
            <img src="assets/mascota.png" alt="Mascota preview"/>
          </div>

          <div style="margin-top:12px">
            <div style="display:flex;gap:10px;align-items:center">
              <div style="flex:1">
                <h4 style="margin:0">Instalación rápida</h4>
                <p class="muted" style="margin:6px 0 0 0;font-size:13px">Clona el repo y abre <code>README.html</code> en el navegador.</p>
              </div>
            </div>

            <div class="grid-cta" style="margin-top:12px">
              <div class="cta">
                <strong>🏁 Quick start</strong>
                <span class="muted">git clone ...</span>
              </div>
              <div class="cta">
                <strong>🧰 Docker</strong>
                <span class="muted">Dockerfile listo</span>
              </div>
              <div class="cta">
                <strong>📦 Assets</strong>
                <span class="muted">Carpeta preparada</span>
              </div>
            </div>
          </div>
        </div>

        <div class="card" style="margin-top:12px;padding:12px">
          <h4 style="margin:0 0 8px 0">Contacto</h4>
          <p class="muted" style="margin:0 0 6px 0">Mantén tu repo ordenado: crea issues para ideas y PRs para cambios.</p>
          <p class="muted" style="margin:0;font-size:13px">© Project_58</p>
        </div>
      </aside>
    </main>

    <footer>
      Tip: si quieres que esto quede visible en la página principal de GitHub, sube el archivo <code>README.md</code> con una imagen/PNG exportada o agrega <code>index.html</code> y activa GitHub Pages en la rama <code>main</code>.
    </footer>
  </div>
</body>
</html>
