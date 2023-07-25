const ENV = Cypress.env();

describe("Login", () => {
  it("shouldn't log in", () => {
    cy.visit(`${ENV.HOST}login`);

    cy.contains("EN").click();
    cy.contains("Forgot password");

    cy.contains("ES").click();
    cy.contains("Olvido su contraseña?");

    cy.contains("PT").click();
    cy.contains("Esqueceu a senha?");
  });
});
