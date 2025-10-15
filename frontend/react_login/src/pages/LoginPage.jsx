import LoginForm from "../components/LoginForm";

function LoginPage() {
  const handleLogin = async ({ email, password }) => {
    console.log("Datos recibidos:", email, password);
    try {
      const res = await fetch("http://127.0.0.1:8080/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email, password }),
      });

      if (!res.ok) {
        const errData = await res.json();
        alert(errData.message || "Error al iniciar sesión");
        return;
      }

      const data = await res.json();
      console.log("Login exitoso:", data);
      // Aquí podrías redirigir al usuario o guardar el token
    } catch (error) {
      console.error("Error en login:", error);
      alert("No se pudo conectar al servidor");
    }
  };

  const handleSignup = async ({
    nombre,
    apellidos,
    edad,
    dni,
    genero,
    grado,
    email,
    password,
  }) => {
    console.log("Datos recibidos:", email, password);

    try {
      const res = await fetch("http://127.0.0.1:8080/signup", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          nombre,
          apellidos,
          edad: parseInt(edad, 10),
          dni: parseInt(dni, 10),
          genero,
          grado: parseInt(grado, 10),
          email,
          password,
        }),
      });

      if (res.status === 409) {
        const { message } = await res.json();
        alert(message || "El usuario ya existe");
        return;
      }

      if (!res.ok) {
        const { message } = await res.json();
        alert(message || "Error al registrarse");
        return;
      }

      const data = await res.json();
      console.log("Registro exitoso:", data);
      alert("Registro exitoso ✅");
      // Podrías limpiar el formulario o redirigir al login
    } catch (error) {
      console.error("Error en signup:", error);
      alert("No se pudo conectar al servidor");
    }
  };

  return (
    <div className="relative min-h-screen flex items-center justify-center bg-orange-50 overflow-hidden">
      <svg
        className="absolute bottom-0 left-0 w-full"
        viewBox="0 0 1440 320"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          fill="#fb923c"
          fillOpacity="0.2"
          d="M0,160L60,170.7C120,181,240,203,360,213.3C480,224,600,224,720,192C840,160,960,96,1080,80C1200,64,1320,96,1380,112L1440,128L1440,320L1380,320C1320,320,1200,320,1080,320C960,320,840,320,720,320C600,320,480,320,360,320C240,320,120,320,60,320L0,320Z"
        />
      </svg>

      <div className="relative z-10">
        <LoginForm onLogin={handleLogin} onSignup={handleSignup} />
      </div>
    </div>
  );
}

export default LoginPage;
