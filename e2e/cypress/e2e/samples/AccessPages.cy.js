const ENV = Cypress.env();

describe("Access Pages", () => {
  it("navigate all pages", () => {
    cy.visit(`${ENV.HOST}login`);

    cy.get("#input-email").type("andre.pinto@mariawl.com");
    cy.get("#input-password").type("Fluke@12345");
    cy.get("#btn-login").click();

    cy.location("pathname").should("include", "/courses");

    cy.get(".courses-header-tabs .a-course-list-my-certificates").click();
    cy.contains("teste pt");

    cy.get(".courses-header-tabs .a-course-list-my-courses").click();
    cy.contains("Ciudad de Mexico");

    cy.get(".courses-header-tabs .a-course-list").click();
    cy.contains("Fluke training Program Level 2");
    cy.contains("Materiales");

    cy.get(".menu-header-anchor.user-list").click();
    cy.contains("Estudantes");
    cy.contains("Masters");
    cy.contains("TSMs");
    cy.contains("Admins");

    cy.get(".menu-header-anchor.team-list").click();
    cy.contains("MariaWL");
    cy.contains("Fluke Brasil - KAM");

    cy.get(".menu-header-anchor.learning-paths-list").click();
    cy.contains("teste 1");

    cy.get(".menu-header-anchor.report-list").click();
    cy.contains("cursos");
    cy.contains("Estudantes");
    cy.contains("Exportar XLS", { timeout: 15000 });

    cy.get(".menu-header-anchor.setting-list").click();
    cy.contains("definições");
    cy.contains("Gerar certificado customizado", { timeout: 15000 });
  });
});
