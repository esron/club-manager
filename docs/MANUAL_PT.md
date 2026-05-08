# Gestor do Clube - Manual do Usuário

**Versão:** 1.0.0  
**Data:** Maio 2026

## Índice

1. [Introdução](#introdução)
2. [Instalação](#instalação)
3. [Primeiro Uso](#primeiro-uso)
4. [Gerenciar Membros](#gerenciar-membros)
5. [Registrar Pagamentos](#registrar-pagamentos)
6. [Visualizar Dívidas](#visualizar-dívidas)
7. [Exportar Relatórios](#exportar-relatórios)
8. [Configurações](#configurações)
9. [Segurança](#segurança)
10. [Perguntas Frequentes](#perguntas-frequentes)

---

## Introdução

O **Gestor do Clube** é um aplicativo desktop para gerenciar mensalidades de clubes de forma simples e segura. Todos os seus dados são criptografados e protegidos por senha.

### Características Principais

- ✅ **100% Offline** - Não precisa de internet
- 🔒 **Criptografia Total** - Dados protegidos com senha
- 💰 **Gestão de Pagamentos** - Registre mensalidades por mês
- 📊 **Cálculo Automático de Dívidas** - Veja quem está em débito
- 📈 **Dashboard Visual** - Gráficos e resumos financeiros
- 📥 **Exportação** - Exporte para Excel (.xlsx) ou CSV
- 🔍 **Busca Rápida** - Encontre membros facilmente
- 🇧🇷 **Interface em Português** - Totalmente em português brasileiro

---

## Instalação

### Windows

1. Baixe o instalador: `Gestor do Clube_1.0.0_x64-setup.exe`
2. Execute o instalador
3. Siga as instruções na tela
4. Após instalação, o atalho aparecerá no Menu Iniciar

### Linux

1. Baixe o AppImage: `gestor-do-clube_1.0.0_amd64.AppImage`
2. Torne executável:
   ```bash
   chmod +x gestor-do-clube_1.0.0_amd64.AppImage
   ```
3. Execute:
   ```bash
   ./gestor-do-clube_1.0.0_amd64.AppImage
   ```

**Dica:** Você pode mover o AppImage para `~/Applications` ou `/opt` para fácil acesso.

---

## Primeiro Uso

### 1. Criar Senha

Na primeira vez que abrir o aplicativo, você precisará criar uma senha:

1. Digite uma senha forte (mínimo 8 caracteres)
2. Confirme a senha
3. Clique em "Criar"

⚠️ **IMPORTANTE:** Não há recuperação de senha! Se esquecer sua senha, seus dados serão perdidos. Guarde em local seguro.

### 2. Configuração Inicial

Após criar a senha, você será direcionado ao Dashboard. Recomendamos:

1. Ir em **Configurações** (ícone de engrenagem)
2. Definir a **Mensalidade Mínima** (padrão: R$ 15,00)
3. Salvar configurações

---

## Gerenciar Membros

### Adicionar Novo Membro

1. Clique em **Membros** no menu lateral
2. Clique em **[+ Adicionar Membro]**
3. Preencha:
   - **Nome:** Nome completo do membro
   - **Data de Início:** Quando começou a pagar mensalidades
4. Clique em **Adicionar**

### Visualizar Membros

A lista de membros mostra:
- Nome do membro
- Data de início
- Dívida atual (se houver)
- Botão **[Ver]** para detalhes

**Membros com dívida aparecem em vermelho.**

### Ver Detalhes do Membro

Clique no nome de um membro para ver:
- Histórico completo de pagamentos
- Meses não pagos
- Dívida total acumulada
- Opção de adicionar pagamento direto

### Buscar Membros

Use a barra de busca no topo da lista:
- Digite parte do nome
- A lista filtra automaticamente
- Clique no **[X]** para limpar a busca

### Desativar Membro

1. Abra os detalhes do membro
2. Clique em **[Desativar]**
3. Confirme a ação

**Nota:** Membros desativados não aparecem na lista principal, mas seus dados são preservados.

---

## Registrar Pagamentos

### Adicionar Pagamento (Método 1: Toolbar)

1. Clique no botão **[+ Adicionar Pagamento]** no topo da tela
2. Selecione o **Membro**
3. Selecione o **Mês** de referência
4. Selecione o **Ano**
5. Digite o **Valor** (padrão: mensalidade mínima)
6. Escolha a **Data do Pagamento** (padrão: hoje)
7. Clique em **[Salvar]**

### Adicionar Pagamento (Método 2: Membro)

1. Vá em **Membros**
2. Clique no nome do membro
3. Na seção "Meses Não Pagos", clique em **[+ Adicionar Pagamento]**
4. O mês será preenchido automaticamente
5. Digite o valor e confirme

### Regras de Pagamento

- ✅ Pode pagar qualquer mês (passado, presente ou futuro)
- ✅ Pode pagar mais de um mês para o mesmo membro
- ❌ Não pode pagar o mesmo mês duas vezes
- ✅ Valor pode ser diferente da mensalidade mínima

---

## Visualizar Dívidas

### Como Funciona o Cálculo

Um membro tem dívida quando:
1. Não pagou a mensalidade de um mês
2. Já passou o dia 10 do mês seguinte

**Exemplo:**
- Mês de Março sem pagamento
- Hoje é 15 de Abril
- Débito: R$ 15,00 (mensalidade mínima)

### Dashboard

O Dashboard mostra:
- **Dívida Total do Clube:** Soma de todas as dívidas
- **Membros Ativos:** Quantidade de membros
- **Gráfico de Pagamentos:** Últimos 6 meses
- **Evolução da Dívida:** Tendência nos últimos 6 meses

### Detalhes por Membro

Cada membro mostra:
- **Dívida Atual:** Valor total em atraso
- **Meses Não Pagos:** Lista detalhada com valores
- **Opção de Pagamento Rápido:** Pagar diretamente

---

## Exportar Relatórios

### Tipos de Relatório

1. **Status de Dívidas**
   - Lista de membros e suas dívidas atuais
   - Quantidade de meses em atraso

2. **Histórico de Pagamentos**
   - Matriz mês a mês de todos os pagamentos
   - Visualização completa de todo o período

### Exportar Passo a Passo

1. Clique em **Relatórios** no menu
2. Escolha o **Tipo de Relatório**
3. Configure opções:
   - **Período:** Data inicial e final (para Histórico)
   - **Incluir inativos:** Membros desativados (para Dívidas)
   - **Formato:** CSV ou XLSX (Excel)
   - **Anonimizar:** Ocultar nomes (aparecem como "Membro #1")
4. Clique em **[Visualizar]** para pré-visualizar
5. Clique em **[Exportar]**
6. Digite sua senha novamente (segurança)
7. Escolha onde salvar o arquivo
8. Arquivo criado!

### Formatos de Arquivo

**XLSX (Excel):**
- Abre no Excel, LibreOffice Calc, Google Sheets
- Formatação visual (cores, negrito)
- Ideal para apresentações

**CSV:**
- Arquivo de texto simples
- Compatível com qualquer planilha
- Menor tamanho

**Arquivos anonimizados:**
- Nomes substituídos por "Membro #1", "Membro #2", etc.
- Útil para compartilhar sem expor identidades
- Valores e datas permanecem reais

---

## Configurações

### Mensalidade Mínima

Define o valor padrão da mensalidade:
1. Vá em **Configurações**
2. Edite o campo **Mensalidade Mínima (R$)**
3. Use formato: `15,00` ou `15.00`
4. Clique em **[Salvar]**

**Nota:** Alterar este valor não afeta dívidas já calculadas.

### Alterar Senha

Para trocar sua senha:
1. Vá em **Configurações**
2. Role até **Alterar Senha**
3. Digite a **Senha Atual**
4. Digite a **Nova Senha** (mínimo 8 caracteres)
5. **Confirme a Nova Senha**
6. Clique em **[Alterar Senha]**

✅ Senha alterada! Use a nova senha no próximo login.

---

## Segurança

### Criptografia

- Todos os dados são criptografados com **AES-256**
- A senha nunca é armazenada, apenas um hash
- Banco de dados é protegido por **SQLCipher**

### Boas Práticas

✅ **Fazer:**
- Use senha forte e única
- Anote a senha em local seguro físico
- Faça backup do arquivo `clube.db` regularmente
- Use gerenciador de senhas

❌ **Evitar:**
- Senhas fracas ou obvias
- Compartilhar senha
- Deixar aplicativo aberto sem supervisão
- Exportar relatórios completos sem necessidade

### Backup

O banco de dados fica em:
- **Windows:** `C:\Users\{seu_usuario}\Documents\GestorDoClube\clube.db`
- **Linux:** `~/Documents/GestorDoClube/clube.db`

**Como fazer backup:**
1. Feche o aplicativo
2. Copie o arquivo `clube.db`
3. Cole em local seguro (pendrive, nuvem, etc.)

**Como restaurar:**
1. Feche o aplicativo
2. Substitua `clube.db` pelo backup
3. Abra o aplicativo e faça login

---

## Perguntas Frequentes

### Esqueci minha senha. O que faço?

**Não há recuperação de senha.** Se esqueceu, os dados estão perdidos. É por isso que recomendamos:
- Anotar senha em local físico seguro
- Usar gerenciador de senhas
- Fazer backups regulares

### Posso usar em vários computadores?

Sim! Copie o arquivo `clube.db` para outro computador. Use a mesma senha.

### O aplicativo precisa de internet?

Não. Funciona 100% offline.

### Posso pagar vários meses de uma vez?

Sim, mas precisa registrar um pagamento por mês. Exemplo: pagar 3 meses = fazer 3 registros.

### Como sei se alguém está devendo?

1. Veja o Dashboard - mostra dívida total
2. Veja a lista de Membros - membros em débito aparecem em vermelho
3. Clique no membro para ver detalhes

### Posso mudar o valor da mensalidade?

Sim, em **Configurações > Mensalidade Mínima**. Mas isso não altera dívidas já calculadas.

### O que acontece se eu desativar um membro?

Ele sai da lista principal, mas os dados ficam salvos. Você pode exportar relatórios incluindo inativos.

### Posso voltar atrás em um pagamento?

Sim. Vá nos detalhes do membro, encontre o pagamento e clique em **[Excluir]**.

### Como faço gráficos personalizados?

Exporte para Excel/CSV e crie gráficos na planilha.

### O aplicativo funciona em Mac?

Não nesta versão. Apenas Windows e Linux.

---

## Suporte

**Problemas ou dúvidas?**

- Verifique este manual primeiro
- Clique em **Ajuda** dentro do aplicativo
- Reporte bugs no GitHub: [github.com/essilva/club-manager](https://github.com/essilva/club-manager)

**Versão do aplicativo:** 1.0.0

---

**Gestor do Clube** - Gestão simples e segura de mensalidades.
