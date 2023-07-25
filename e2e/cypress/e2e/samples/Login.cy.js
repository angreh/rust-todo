const ENV = Cypress.env();

describe("Login", () => {
  it("shouldn't log in", () => {
    cy.visit(`${ENV.HOST}login`);

    cy.get("#input-email").type("andre.pinto@mariawl.com");
    cy.get("#input-password").type("asdasdasd");
    cy.get("#btn-login").click();

    cy.contains("Invalid");
  });

  it("should log in", () => {
    cy.visit(`${ENV.HOST}login`);

    cy.get("#input-email").type("andre.pinto@mariawl.com");
    cy.get("#input-password").type("Fluke@12345");
    cy.get("#btn-login").click();

    cy.location("pathname").should("include", "/courses");
  });
});
