<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CoproprieteBimensuel</name>
   <tag></tag>
   <elementGuidId>b46e1020-7982-485b-b41d-5a6cad0db3b8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;crmAccountType\&quot;: \&quot;CA_UA\&quot;,\n    \&quot;crmParentCode\&quot;: \&quot;${GEST_codeGestionnaire}\&quot;,\n    \&quot;code\&quot;: \&quot;${codePropriétaire}\&quot;,\n    \&quot;description\&quot;: \&quot;${descriptionName}\&quot;,\n    \&quot;externalRef1\&quot;: \&quot;EXT001\&quot;,\n    \&quot;externalRef2\&quot;: \&quot;EXT002\&quot;,\n    \&quot;email\&quot;: \&quot;${email}\&quot;,\n    \&quot;address\&quot;: {\n        \&quot;address1\&quot;: \&quot;${address1}\&quot;,\n        \&quot;address2\&quot;: \&quot;${address2}\&quot;,\n        \&quot;address3\&quot;: \&quot;${address3}\&quot;,\n        \&quot;zipCode\&quot;: \&quot;${zipCode}\&quot;,\n        \&quot;city\&quot;: \&quot;${ville}\&quot;,\n        \&quot;country\&quot;: \&quot;France\&quot;,\n        \&quot;state\&quot;: \&quot;\&quot;\n    },\n    \&quot;contactInformation\&quot;: {\n        \&quot;email\&quot;: \&quot;${contactInfoEmail}\&quot;,\n        \&quot;phone\&quot;: \&quot;${contactInfoPhone}\&quot;,\n        \&quot;mobile\&quot;: \&quot;${contactInfoMobile}\&quot;,\n        \&quot;fax\&quot;: \&quot;${contactInfoFax}\&quot;\n    },\n    \&quot;jobTitle\&quot;: \&quot;Manager\&quot;,\n    \&quot;language\&quot;: \&quot;FRA\&quot;,\n    \&quot;seller\&quot;: \&quot;AGENCE_Toulouse\&quot;,\n     \&quot;legalEntityType\&quot;: {\n        \&quot;code\&quot;: \&quot;SA\&quot;\n\n    },\n    \&quot;registrationNumbers\&quot;: [\n        {\n            \&quot;registrationNo\&quot;: \&quot;9517452\&quot;,\n            \&quot;isoIcdCode\&quot;: \&quot;0009\&quot;\n        }\n    ],\n    \&quot;vatNo\&quot;: \&quot;FR12345678900\&quot;,\n    \&quot;mandateIdentification\&quot;: \&quot;MANDAT001\&quot;,\n    \&quot;mandateDate\&quot;: \&quot;2025-02-03\&quot;,\n    \&quot;currency\&quot;: \&quot;EUR\&quot;,\n    \&quot;creditCategory\&quot;: \&quot;PRO\&quot;,\n    \&quot;methodOfPayment\&quot;: [\n        {\n            \&quot;paymentMethodType\&quot;: \&quot;DIRECTDEBIT\&quot;,\n            \&quot;bankCoordinates\&quot;: {\n                \&quot;bankCode\&quot;: \&quot;30004\&quot;,\n                \&quot;bankName\&quot;: \&quot;central\&quot;,\n                \&quot;branchCode\&quot;: \&quot;00001\&quot;,\n                \&quot;accountOwner\&quot;: \&quot;${proprioCompteBancaire}\&quot;,\n                \&quot;accountNumber\&quot;: \&quot;00000000001\&quot;,\n                \&quot;key\&quot;: \&quot;21\&quot;,\n                \&quot;iban\&quot;: \&quot;FR7630004000010000000000121\&quot;,\n                \&quot;bic\&quot;: \&quot;BNPAFRPP\&quot;\n            },\n            \&quot;mandateIdentification\&quot;: \&quot;MANDAT001\&quot;,\n            \&quot;alias\&quot;: \&quot;DIRECTDEBIT00000001\&quot;,\n            \&quot;preferred\&quot;: true\n        },\n        {\n            \&quot;paymentMethodType\&quot;: \&quot;WIRETRANSFER\&quot;,\n            \&quot;alias\&quot;: \&quot;WIRETRANSFER00000001\&quot;,\n            \&quot;preferred\&quot;: false\n        },\n        {\n            \&quot;paymentMethodType\&quot;: \&quot;CHECK\&quot;,\n            \&quot;alias\&quot;: \&quot;CHECK00000001\&quot;,\n            \&quot;preferred\&quot;: false\n        }\n    ],\n    \&quot;isCompany\&quot;: true,\n    \&quot;billingCycle\&quot;: \&quot;CYC_FACTU_BIMENSUEL\&quot;,\n    \&quot;country\&quot;: \&quot;FR\&quot;,\n    \&quot;taxCategoryCode\&quot; : \&quot;TAXCAT_ASSUJ\&quot;,\n    \&quot;electronicBilling\&quot;: true,\n       \&quot;customFields\&quot;: {\n        \&quot;customField\&quot;: [\n          {\n                \&quot;code\&quot;: \&quot;CF_REFERENCE_INFORMATIQUE\&quot;,\n                \&quot;stringValue\&quot;: \&quot;REF-INF-444\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_CODE_INTERCO_ANAEL\&quot;,\n                \&quot;stringValue\&quot;: \&quot;ANAEL-456\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_COMMENTAIRE_INTERNE\&quot;,\n                \&quot;stringValue\&quot;: \&quot;Client premium - Contrat spécial\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_SELLER_ID\&quot;,\n                 \&quot;entityReferenceValue\&quot;: {\n                    \&quot;classname\&quot;: \&quot;org.meveo.model.admin.Seller\&quot;,\n                    \&quot;code\&quot;: \&quot;AGENCE_Toulouse\&quot;,\n                    \&quot;id\&quot;: 38\n                }\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_NUMERO_COMPTE_CLIENT\&quot;,\n                \&quot;stringValue\&quot;: \&quot;CLI2025-001234\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_CODE_POSTAL\&quot;,\n                \&quot;stringValue\&quot;: \&quot;75008\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_ADRESSE_SPECIFIQUE_FACTURATION\&quot;,\n                \&quot;booleanValue\&quot;: true\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_ADRESSE_2\&quot;,\n                \&quot;stringValue\&quot;: \&quot;Bâtiment B\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_NOM_DU_GESTIONNAIRE\&quot;,\n                \&quot;stringValue\&quot;: \&quot;Sophie Bernard\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_ADRESSE_3\&quot;,\n                \&quot;stringValue\&quot;: \&quot;Code 4521\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_ADRESSE_1\&quot;,\n                \&quot;stringValue\&quot;: \&quot;CF_AFFICHAGE_COPRO_FACTURE_REGROUPEMENT\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;${nomDeLaVille}\&quot;,\n                \&quot;stringValue\&quot;: \&quot;Paris\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;${nomDeLaCopropriete}\&quot;,\n                \&quot;stringValue\&quot;: \&quot;Résidence Les Jardins de Paris\&quot;\n            }\n        ]\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>da00ab44-a2f5-42af-9cd8-7927103ac236</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic b3BlbmNlbGwuYWRtaW46b3BlbmNlbGwuYWRtaW4=</value>
      <webElementGuid>1b153fc5-58ac-475c-841a-89ce2fbed7e8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.BASE_URL_API}/account/accountHierarchy/createOrUpdateCRMAccountHierarchy</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>619fc0d8-5b20-48e1-a7d4-95a5306d9038</id>
      <masked>false</masked>
      <name>codePropriétaire</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3d2dce69-97e0-4b60-a888-5291551b1f7e</id>
      <masked>false</masked>
      <name>zipCode</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>79494f0c-67a6-40bc-b210-9bcaeb62a975</id>
      <masked>false</masked>
      <name>descriptionName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4b6e2503-5ef6-4491-8404-42c9dfa13ad0</id>
      <masked>false</masked>
      <name>contactInfoEmail</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>91f291a0-48e0-41d3-a0c9-74bf714f0687</id>
      <masked>false</masked>
      <name>ville</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7135f9a2-fcc9-47ca-93c4-76ef8b27c94f</id>
      <masked>false</masked>
      <name>address1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ac460a47-875a-4903-a230-2ff83cb152e8</id>
      <masked>false</masked>
      <name>address2</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bfdac925-1075-4845-886b-224dfe228516</id>
      <masked>false</masked>
      <name>address3</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8b08a1e7-9b27-40bf-be98-59afd321a813</id>
      <masked>false</masked>
      <name>contactInfoPhone</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f9964150-ce2a-4a3e-9d4d-8118d4245a61</id>
      <masked>false</masked>
      <name>contactInfoMobile</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1edb9825-c4f6-451f-925d-93ae666c77ec</id>
      <masked>false</masked>
      <name>contactInfoFax</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>225c1d45-2949-4b28-87fe-b03c5b1ef2f2</id>
      <masked>false</masked>
      <name>proprioCompteBancaire</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f2f27754-40e8-4cfb-a1e7-c16fb48e10de</id>
      <masked>false</masked>
      <name>nomDeLaCopropriete</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>945f3720-9bb4-49b1-a922-af5fd53e7b03</id>
      <masked>false</masked>
      <name>nomDeLaVille</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>79df2c88-3787-43bc-be02-ed1256d49cb5</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d05b7b70-6941-4e62-bbe1-3e6ee49c9917</id>
      <masked>false</masked>
      <name>GEST_codeGestionnaire</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
