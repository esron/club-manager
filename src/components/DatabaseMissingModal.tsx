import { useState } from 'react';
import { useAuth } from '../contexts/AuthContext';

export const DatabaseMissingModal = () => {
  const { initializeDatabase } = useAuth();
  const [isCreating, setIsCreating] = useState(false);
  const [error, setError] = useState('');

  const handleCreateNew = async () => {
    setIsCreating(true);
    setError('');
    try {
      await initializeDatabase();
    } catch (err) {
      console.error('Error creating database:', err);
      setError('Erro ao criar banco de dados');
      setIsCreating(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
      <div className="bg-dark-surface p-8 rounded-lg border border-dark-border max-w-md w-full">
        <h2 className="text-xl font-bold mb-4 text-dark-text-primary">Banco de Dados Não Encontrado</h2>
        <p className="text-dark-text-secondary mb-4">
          O arquivo de banco de dados não foi encontrado ou está corrompido.
        </p>
        <p className="text-dark-text-secondary mb-6">
          Você pode estar restaurando um backup ou migrando para uma nova instalação.
          Se você deseja criar um novo banco de dados vazio, clique em "Criar Novo".
        </p>

        {error && <p className="text-dark-error mb-4 text-sm">{error}</p>}

        <div className="bg-dark-bg border border-dark-warning p-4 rounded mb-6">
          <p className="text-dark-warning text-sm font-bold mb-2">⚠️ Atenção:</p>
          <p className="text-dark-text-secondary text-sm">
            Criar um novo banco de dados removerá todos os dados existentes.
            Se você tem um backup, feche o aplicativo e restaure o arquivo antes de continuar.
          </p>
          <p className="text-dark-text-secondary text-sm mt-2">
            Localização do banco: <code className="text-dark-text-primary">~/.local/share/GestorDoClube/club.db</code>
          </p>
        </div>

        <button
          onClick={handleCreateNew}
          disabled={isCreating}
          className="w-full bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90 disabled:opacity-50"
        >
          {isCreating ? 'Criando...' : 'Criar Novo Banco de Dados'}
        </button>
      </div>
    </div>
  );
};
