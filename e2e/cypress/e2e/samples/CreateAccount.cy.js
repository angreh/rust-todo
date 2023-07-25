const ENV = Cypress.env();

describe("Create account", () => {
  it("register new user - success", () => {
    cy.visit(`${ENV.HOST}login`);

    cy.contains("Create account").click();

    cy.get("input[name='name']").type("André");
    cy.get("input[name='email']").type("andre.example@gmail.com");
    cy.get("input[name='password']").type("123456");
    cy.get("input[name='passwordConfirmation']").type("123456");

    cy.get(".v-select:has(input[name='country'])").click();
    cy.contains("BRAZIL").click();

    cy.get(".v-select:has(input[name='master'])").click();
    cy.contains("Fluke Brasil").click();

    cy.get(".v-input:has(input[name='privacy'])").click();

    cy.get("button[name='register']").click();

    cy.contains("Registration successfully");
  });

  it("register new user - fail | same email", () => {
    cy.visit(`${ENV.HOST}login`);

    cy.contains("Create account").click();

    cy.get("input[name='name']").type("André");
    cy.get("input[name='email']").type("andre.example@gmail.com");
    cy.get("input[name='password']").type("123456");
    cy.get("input[name='passwordConfirmation']").type("123456");

    cy.get(".v-select:has(input[name='country'])").click();
    cy.contains("BRAZIL").click();

    cy.get(".v-select:has(input[name='master'])").click();
    cy.contains("Fluke Brasil").click();

    cy.get(".v-input:has(input[name='privacy'])").click();

    cy.get("button[name='register']").click();

    cy.contains("Unable to register");
  });
});
