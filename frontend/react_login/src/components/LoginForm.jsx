import { useState } from "react";

function AuthForm({ onLogin, onSignup }) {
  const [isSignup, setIsSignup] = useState(false);

  // Campos comunes
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  // Campos de registro
  const [nombre, setNombre] = useState("");
  const [apellidos, setApellidos] = useState("");
  const [edad, setEdad] = useState("");
  const [dni, setDni] = useState("");
  const [genero, setGenero] = useState("");
  const [grado, setGrado] = useState("");


  const handleSubmit = (e) => {
    e.preventDefault();
    if (isSignup) {
      onSignup({
        nombre,
        apellidos,
        edad,
        dni,
        genero,
        grado,
        email,
        password,
      });
    } else {
      onLogin({ email, password });
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gradient-to-br p-4">
      {/* LOGO / TÍTULO */}
      <h1 className="text-4xl font-extrabold text-orange-600 mb-6 drop-shadow-sm tracking-wide">
        Project <span className="text-orange-500">58</span>
      </h1>

      <form
        onSubmit={handleSubmit}
        className="bg-white shadow-xl rounded-2xl p-8 w-full max-w-md border border-orange-100"
      >
        <h2 className="text-2xl font-bold text-orange-600 text-center mb-6">
          {isSignup ? "Crear cuenta" : "Iniciar sesión"}
        </h2>

        {/* CAMPOS DE SIGNUP */}
        {isSignup && (
          <>
            {/* Nombres */}
            <div className="mb-3">
              <label className="block text-sm font-medium text-gray-600 mb-1">
                Nombres
              </label>
              <input
                type="text"
                value={nombre}
                onChange={(e) => setNombre(e.target.value)}
                className="w-full border border-orange-200 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-orange-400"
                required
              />
            </div>

            {/* Apellidos */}
            <div className="mb-3">
              <label className="block text-sm font-medium text-gray-600 mb-1">
                Apellidos
              </label>
              <input
                type="text"
                value={apellidos}
                onChange={(e) => setApellidos(e.target.value)}
                className="w-full border border-orange-200 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-orange-400"
                required
              />
            </div>

            {/* Edad */}
            <div className="mb-3">
              <label className="block text-sm font-medium text-gray-600 mb-1">
                Edad
              </label>
              <input
                type="number"
                value={edad}
                onChange={(e) => setEdad(e.target.value)}
                className="w-full border border-orange-200 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-orange-400"
                required
                min="1"
              />
            </div>

            {/* DNI */}
            <div className="mb-3">
              <label className="block text-sm font-medium text-gray-600 mb-1">
                DNI
              </label>
              <input
                type="text"
                value={dni}
                onChange={(e) => setDni(e.target.value)}
                className="w-full border border-orange-200 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-orange-400"
                required
              />
            </div>

            {/* Género */}
            <div className="mb-3">
              <label className="block text-sm font-medium text-gray-600 mb-1">
                Género
              </label>
              <select
                value={genero}
                onChange={(e) => setGenero(e.target.value)}
                className="w-full border border-orange-200 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-orange-400 bg-white"
                required
              >
                <option value="">Selecciona tu género</option>
                <option value="Femenino">Femenino</option>
                <option value="Masculino">Masculino</option>
                <option value="Joselito">Joselito</option>
              </select>
            </div>

            {/* Grado */}
            <div className="mb-3">
              <label className="block text-sm font-medium text-gray-600 mb-1">
                Grado
              </label>
              <select
                value={grado}
                onChange={(e) => setGrado(e.target.value)}
                className="w-full border border-orange-200 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-orange-400 bg-white"
                required
              >
                <option value="">Selecciona tu grado</option>
                {[1, 2, 3, 4, 5].map((num) => (
                  <option key={num} value={num}>
                    {num}
                  </option>
                ))}
              </select>
            </div>
          </>
        )}

        {/* CAMPOS COMUNES */}
        <div className="mt-4">
          <label className="block text-sm font-medium text-gray-600 mb-1">
            Correo electrónico
          </label>
          <input
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            className="w-full border border-orange-200 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-orange-400"
            required
          />
        </div>

        <div className="mt-4">
          <label className="block text-sm font-medium text-gray-600 mb-1">
            Contraseña
          </label>
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            className="w-full border border-orange-200 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-orange-400"
            required
          />
        </div>

        {/* BOTÓN */}
        <button
          type="submit"
          className="w-full mt-6 bg-orange-500 text-white font-semibold py-2 rounded-lg hover:bg-orange-600 transition-all duration-200 shadow-md"
        >
          {isSignup ? "Registrarse" : "Entrar"}
        </button>

        {/* CAMBIO ENTRE LOGIN / SIGNUP */}
        <p className="text-center text-sm text-gray-600 mt-4">
          {isSignup ? (
            <>
              ¿Ya tienes cuenta?{" "}
              <button
                type="button"
                onClick={() => setIsSignup(false)}
                className="text-orange-600 font-medium hover:underline"
              >
                Iniciar sesión
              </button>
            </>
          ) : (
            <>
              ¿No tienes cuenta?{" "}
              <button
                type="button"
                onClick={() => setIsSignup(true)}
                className="text-orange-600 font-medium hover:underline"
              >
                Crear una
              </button>
            </>
          )}
        </p>
      </form>
    </div>
  );
}

export default AuthForm;
