<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>select_-- Pilih --BasicIntermediateAdvanceExpert</name>
   <tag></tag>
   <elementGuidId>53f623b1-fb62-493b-8448-cab0349f2d32</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//select[@id='selectSkillLevel--1']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#selectSkillLevel--1</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>#selectSkillLevel--1</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>select</value>
      <webElementGuid>7603a236-258d-40f4-8bca-e1c07f9d9f67</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-collapse</name>
      <type>Main</type>
      <value>#collapseSkill--1</value>
      <webElementGuid>816f3747-c39f-49f1-91f3-8a51341ccf64</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>selectSkillLevel--1</value>
      <webElementGuid>ec2771cc-634a-408d-80f6-c488dd675383</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>custom-select required</value>
      <webElementGuid>0ae70b65-0252-4121-a3f6-8edebc57a11e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onchange</name>
      <type>Main</type>
      <value>
                if($(this).val() === ''){
                    $('#txtSkillLevel--1').text($(this).val()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                    $('#frame').contents().find('#txtListSkillLevel--1').text($(this).val()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                    $('#txtListSkillLevel--1').text($(this).val()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                } else {
                    $('#txtSkillLevel--1').text('('+$(this).find('option:selected').text()+')').fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                    $('#frame').contents().find('#txtListSkillLevel--1').text('('+$(this).find('option:selected').text()+')').fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                    $('#txtListSkillLevel--1').text('('+$(this).find('option:selected').text()+')').fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                }
                isSkillValueBlank(); autoSave(); validateRequire($(this))</value>
      <webElementGuid>59c554a6-666f-40cf-a58e-04443100fd14</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                -- Pilih --BasicIntermediateAdvanceExpert
              </value>
      <webElementGuid>c9e67a20-0dc5-4fee-b178-2546ef79bd10</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;selectSkillLevel--1&quot;)</value>
      <webElementGuid>95ecbbc1-e6c0-4898-ab97-07c5e0c63691</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//select[@id='selectSkillLevel--1']</value>
      <webElementGuid>81144547-4dce-43e4-881b-316a40cc1f41</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='collapseSkill--1']/div[2]/div[2]/select</value>
      <webElementGuid>ad4e1e81-99ed-4d27-9885-d6a7708d59d9</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Tingkat'])[4]/following::select[1]</value>
      <webElementGuid>2577d670-e8de-4cfc-aa42-b2d0a4395c59</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Skill'])[1]/following::select[1]</value>
      <webElementGuid>532898b1-4c67-4650-a20f-efa36cb58396</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Kategori'])[1]/preceding::select[1]</value>
      <webElementGuid>a8729583-cff4-463d-9c95-c809b95a75ad</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Tambah Data'])[3]/preceding::select[2]</value>
      <webElementGuid>a036f7c0-2831-42c8-ac3b-d14c79bf1c5b</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[4]/div/div/div[2]/div[2]/div[2]/select</value>
      <webElementGuid>31c843da-ca8a-4ce3-a244-f746c4e8a80a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//select[@id = 'selectSkillLevel--1' and (text() = '
                -- Pilih --BasicIntermediateAdvanceExpert
              ' or . = '
                -- Pilih --BasicIntermediateAdvanceExpert
              ')]</value>
      <webElementGuid>5ae176ba-1427-4c82-9098-2551be147ff4</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
